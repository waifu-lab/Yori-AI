use std::borrow::Cow;
use std::collections::HashMap;

use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use crate::drawable::{BlendMode, DrawableBuffers, Vertex};
use crate::model::DrawableSnapshot;

const SHADER: &str = include_str!("shader/live2d.wgsl");
const MASK_SHADER: &str = include_str!("shader/mask.wgsl");

const MASK_TEX_SIZE: u32 = 256;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct MvpUniform {
    mvp: [[f32; 4]; 4],
}

fn default_mvp() -> [[f32; 4]; 4] {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

fn blend_normal() -> wgpu::BlendState {
    wgpu::BlendState {
        color: wgpu::BlendComponent {
            src_factor: wgpu::BlendFactor::SrcAlpha,
            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
            operation: wgpu::BlendOperation::Add,
        },
        alpha: wgpu::BlendComponent {
            src_factor: wgpu::BlendFactor::One,
            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
            operation: wgpu::BlendOperation::Add,
        },
    }
}

fn blend_additive() -> wgpu::BlendState {
    wgpu::BlendState {
        color: wgpu::BlendComponent {
            src_factor: wgpu::BlendFactor::SrcAlpha,
            dst_factor: wgpu::BlendFactor::One,
            operation: wgpu::BlendOperation::Add,
        },
        alpha: wgpu::BlendComponent {
            src_factor: wgpu::BlendFactor::Zero,
            dst_factor: wgpu::BlendFactor::One,
            operation: wgpu::BlendOperation::Add,
        },
    }
}

fn blend_multiplicative() -> wgpu::BlendState {
    wgpu::BlendState {
        color: wgpu::BlendComponent {
            src_factor: wgpu::BlendFactor::Dst,
            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
            operation: wgpu::BlendOperation::Add,
        },
        alpha: wgpu::BlendComponent {
            src_factor: wgpu::BlendFactor::Zero,
            dst_factor: wgpu::BlendFactor::One,
            operation: wgpu::BlendOperation::Add,
        },
    }
}

struct ClipGroup {
    mask_source_indices: Vec<usize>,
}

fn build_clip_groups(snapshots: &[DrawableSnapshot]) -> (Vec<ClipGroup>, Vec<Option<usize>>) {
    let mut group_map: HashMap<Vec<usize>, usize> = HashMap::new();
    let mut groups: Vec<ClipGroup> = Vec::new();
    let mut drawable_to_group: Vec<Option<usize>> = vec![None; snapshots.len()];

    for (i, s) in snapshots.iter().enumerate() {
        if s.masks.is_empty() {
            continue;
        }
        let mut key = s.masks.clone();
        key.sort_unstable();

        let cg_idx = *group_map.entry(key.clone()).or_insert_with(|| {
            let idx = groups.len();
            groups.push(ClipGroup {
                mask_source_indices: key,
            });
            idx
        });
        drawable_to_group[i] = Some(cg_idx);
    }
    (groups, drawable_to_group)
}

pub struct Live2dRenderer {
    // ── unmasked pipelines (group layout: 0,1,2) ──
    pipeline_normal: wgpu::RenderPipeline,
    pipeline_add: wgpu::RenderPipeline,
    pipeline_mul: wgpu::RenderPipeline,

    // ── masked pipelines (group layout: 0,1,2,3) ──
    masked_pipeline_normal: wgpu::RenderPipeline,
    masked_pipeline_add: wgpu::RenderPipeline,
    masked_pipeline_mul: wgpu::RenderPipeline,

    // ── mask source render pipeline ──
    mask_pipeline: wgpu::RenderPipeline,

    // ── shared ──
    mvp_buffer: wgpu::Buffer,
    mvp_bind_group: wgpu::BindGroup,

    // ── per-drawable ──
    drawables: Vec<DrawableBuffers>,
    texture_bind_groups: Vec<wgpu::BindGroup>,

    // ── masking ──
    clip_groups: Vec<ClipGroup>,
    /// `drawable_to_group[i]` = clip group index, or None if unmasked.
    drawable_to_group: Vec<Option<usize>>,
    mask_textures: Vec<wgpu::Texture>,
    /// group(3) bind groups for main render pass, one per clip group.
    mask_bind_groups: Vec<wgpu::BindGroup>,
}

impl Live2dRenderer {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        snapshots: &[DrawableSnapshot],
        textures: &[&wgpu::Texture],
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("live2d_shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(SHADER)),
        });
        let mask_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("live2d_mask_shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(MASK_SHADER)),
        });

        let mvp_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("live2d_mvp_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let texture_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("live2d_texture_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let opacity_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("live2d_opacity_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        // group(3): mask texture + sampler
        let mask_sampler_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("live2d_mask_sampler_layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        // ── pipeline layouts ────────────────────────────────────────────────

        let unmasked_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("live2d_unmasked_layout"),
            bind_group_layouts: &[&mvp_layout, &texture_layout, &opacity_layout],
            push_constant_ranges: &[],
        });
        let masked_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("live2d_masked_layout"),
            bind_group_layouts: &[
                &mvp_layout,
                &texture_layout,
                &opacity_layout,
                &mask_sampler_layout,
            ],
            push_constant_ranges: &[],
        });
        let mask_render_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("live2d_mask_render_layout"),
            bind_group_layouts: &[&mvp_layout, &texture_layout],
            push_constant_ranges: &[],
        });

        let primitive = wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            cull_mode: None,
            ..Default::default()
        };

        let make_unmasked = |blend: wgpu::BlendState| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("live2d_unmasked"),
                layout: Some(&unmasked_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[Vertex::LAYOUT],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format,
                        blend: Some(blend),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: Default::default(),
                }),
                primitive,
                depth_stencil: None,
                multisample: Default::default(),
                multiview: None,
                cache: None,
            })
        };

        let make_masked = |blend: wgpu::BlendState| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("live2d_masked"),
                layout: Some(&masked_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[Vertex::LAYOUT],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_masked"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format,
                        blend: Some(blend),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: Default::default(),
                }),
                primitive,
                depth_stencil: None,
                multisample: Default::default(),
                multiview: None,
                cache: None,
            })
        };

        let pipeline_normal = make_unmasked(blend_normal());
        let pipeline_add = make_unmasked(blend_additive());
        let pipeline_mul = make_unmasked(blend_multiplicative());
        let masked_pipeline_normal = make_masked(blend_normal());
        let masked_pipeline_add = make_masked(blend_additive());
        let masked_pipeline_mul = make_masked(blend_multiplicative());

        let mask_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("live2d_mask_render"),
            layout: Some(&mask_render_layout),
            vertex: wgpu::VertexState {
                module: &mask_shader,
                entry_point: Some("vs_mask"),
                buffers: &[Vertex::LAYOUT],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &mask_shader,
                entry_point: Some("fs_mask"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    // Additive: union of multiple mask source alphas
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::One,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::One,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive,
            depth_stencil: None,
            multisample: Default::default(),
            multiview: None,
            cache: None,
        });

        // ── MVP uniform ─────────────────────────────────────────────────────

        let mvp_data = MvpUniform { mvp: default_mvp() };
        let mvp_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("live2d_mvp"),
            contents: bytemuck::bytes_of(&mvp_data),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let mvp_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("live2d_mvp_bg"),
            layout: &mvp_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: mvp_buffer.as_entire_binding(),
            }],
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let texture_bind_groups: Vec<wgpu::BindGroup> = textures
            .iter()
            .map(|tex| {
                let view = tex.create_view(&Default::default());
                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("live2d_texture_bg"),
                    layout: &texture_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&sampler),
                        },
                    ],
                })
            })
            .collect();

        let drawables: Vec<DrawableBuffers> = snapshots
            .iter()
            .map(|s| {
                let vertices = zip_vertices(&s.vertex_positions, &s.uvs);
                DrawableBuffers::new(device, &vertices, &s.indices, &opacity_layout)
            })
            .collect();

        let (clip_groups, drawable_to_group) = build_clip_groups(snapshots);

        // Debug: print clip group statistics
        log::debug!(
            "live2d: {} clip group(s) from {} masked drawable(s)",
            clip_groups.len(),
            drawable_to_group.iter().filter(|g| g.is_some()).count()
        );
        for (i, cg) in clip_groups.iter().enumerate() {
            let any_visible = cg
                .mask_source_indices
                .iter()
                .any(|&idx| snapshots.get(idx).map_or(false, |s| s.is_visible));
            log::debug!(
                "  clip_group[{i}]: {} source(s), any_visible={}",
                cg.mask_source_indices.len(),
                any_visible
            );
        }

        let mask_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let mask_textures: Vec<wgpu::Texture> = clip_groups
            .iter()
            .map(|_| {
                device.create_texture(&wgpu::TextureDescriptor {
                    label: Some("live2d_mask_tex"),
                    size: wgpu::Extent3d {
                        width: MASK_TEX_SIZE,
                        height: MASK_TEX_SIZE,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                        | wgpu::TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                })
            })
            .collect();

        let mask_bind_groups: Vec<wgpu::BindGroup> = mask_textures
            .iter()
            .map(|tex| {
                let view = tex.create_view(&Default::default());
                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("live2d_mask_bg"),
                    layout: &mask_sampler_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&mask_sampler),
                        },
                    ],
                })
            })
            .collect();

        Self {
            pipeline_normal,
            pipeline_add,
            pipeline_mul,
            masked_pipeline_normal,
            masked_pipeline_add,
            masked_pipeline_mul,
            mask_pipeline,
            mvp_buffer,
            mvp_bind_group,
            drawables,
            texture_bind_groups,
            clip_groups,
            drawable_to_group,
            mask_textures,
            mask_bind_groups,
        }
    }

    /// Recompute and upload the MVP so the model fills the viewport at correct
    /// aspect ratio.
    ///
    /// Call once after construction and again whenever the window is resized.
    ///
    /// * `canvas_w`, `canvas_h` — model canvas dimensions in unit space
    ///   (from `Live2dModel::canvas_size_units()`).
    /// * `viewport_w`, `viewport_h` — window/surface size in pixels.
    /// Recompute and upload the MVP.
    ///
    /// * `canvas_w/h`   — model canvas in unit space (`Live2dModel::canvas_size_units`)
    /// * `viewport_w/h` — surface size in pixels
    /// * `zoom`         — additional scale factor (1.0 = fit, >1 = zoom in)
    /// * `pan`          — translation in NDC space `[x, y]` (applied after zoom)
    pub fn set_viewport(
        &self,
        queue: &wgpu::Queue,
        canvas_w: f32,
        canvas_h: f32,
        viewport_w: u32,
        viewport_h: u32,
        zoom: f32,
        pan: [f32; 2],
    ) {
        let canvas_aspect = canvas_w / canvas_h;
        let viewport_aspect = viewport_w as f32 / viewport_h as f32;

        let (sx, sy) = if viewport_aspect >= canvas_aspect {
            let sx = (2.0 / canvas_w) * (canvas_aspect / viewport_aspect);
            let sy = 2.0 / canvas_h;
            (sx, sy)
        } else {
            let sx = 2.0 / canvas_w;
            let sy = (2.0 / canvas_h) * (viewport_aspect / canvas_aspect);
            (sx, sy)
        };

        // Column-major 4×4: scale then translate in NDC.
        let mvp = MvpUniform {
            mvp: [
                [zoom * sx, 0.0, 0.0, 0.0],
                [0.0, zoom * sy, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [pan[0], pan[1], 0.0, 1.0],
            ],
        };
        queue.write_buffer(&self.mvp_buffer, 0, bytemuck::bytes_of(&mvp));
    }

    /// Upload dirty vertex positions and per-drawable params to the GPU.
    /// Call **before** `encode_mask_passes` and `render`.
    pub fn update(&self, queue: &wgpu::Queue, snapshots: &[DrawableSnapshot]) {
        for (buffers, s) in self.drawables.iter().zip(snapshots.iter()) {
            if s.is_vertex_dirty {
                let vertices = zip_vertices(&s.vertex_positions, &s.uvs);
                buffers.update_vertices(queue, &vertices);
            }
            buffers.update_params(queue, s.opacity, s.is_inverted_mask);
        }
    }

    /// Render each clip group's mask sources into their mask textures.
    /// Must be called **before** the main render pass (uses same command encoder).
    pub fn encode_mask_passes(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        snapshots: &[DrawableSnapshot],
    ) {
        for (cg_idx, clip_group) in self.clip_groups.iter().enumerate() {
            let mask_view =
                self.mask_textures[cg_idx].create_view(&wgpu::TextureViewDescriptor::default());

            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("live2d_mask_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &mask_view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            pass.set_pipeline(&self.mask_pipeline);
            pass.set_bind_group(0, &self.mvp_bind_group, &[]);

            for &src_idx in &clip_group.mask_source_indices {
                let Some(snap) = snapshots.get(src_idx) else {
                    continue;
                };
                // Do NOT skip invisible mask sources: they still define the clip region
                // even when the model has their visibility flag cleared.
                let Some(tex_bg) = self.texture_bind_groups.get(snap.texture_index) else {
                    continue;
                };

                pass.set_bind_group(1, tex_bg, &[]);
                let buf = &self.drawables[src_idx];
                pass.set_vertex_buffer(0, buf.vertex_buffer.slice(..));
                pass.set_index_buffer(buf.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                pass.draw_indexed(0..buf.index_count, 0, 0..1);
            }
        }
    }

    /// Record draw calls into an active render pass.
    /// Call `encode_mask_passes` on the same encoder **before** beginning this pass.
    pub fn render<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        snapshots: &[DrawableSnapshot],
    ) {
        let mut order: Vec<usize> = (0..snapshots.len()).collect();
        order.sort_unstable_by_key(|&i| snapshots[i].render_order);

        render_pass.set_bind_group(0, &self.mvp_bind_group, &[]);

        for i in order {
            let s = &snapshots[i];
            if !s.is_visible {
                continue;
            }

            let Some(tex_bg) = self.texture_bind_groups.get(s.texture_index) else {
                log::warn!(
                    "drawable {i}: texture_index {} out of range",
                    s.texture_index
                );
                continue;
            };

            let buf = &self.drawables[i];

            if let Some(&cg_idx) = self.drawable_to_group[i].as_ref() {
                let pipeline = match s.blend_mode {
                    BlendMode::Normal => &self.masked_pipeline_normal,
                    BlendMode::Additive => &self.masked_pipeline_add,
                    BlendMode::Multiplicative => &self.masked_pipeline_mul,
                };
                render_pass.set_pipeline(pipeline);
                render_pass.set_bind_group(1, tex_bg, &[]);
                render_pass.set_bind_group(2, &buf.opacity_bind_group, &[]);
                render_pass.set_bind_group(3, &self.mask_bind_groups[cg_idx], &[]);
            } else {
                let pipeline = match s.blend_mode {
                    BlendMode::Normal => &self.pipeline_normal,
                    BlendMode::Additive => &self.pipeline_add,
                    BlendMode::Multiplicative => &self.pipeline_mul,
                };
                render_pass.set_pipeline(pipeline);
                render_pass.set_bind_group(1, tex_bg, &[]);
                render_pass.set_bind_group(2, &buf.opacity_bind_group, &[]);
            }

            render_pass.set_vertex_buffer(0, buf.vertex_buffer.slice(..));
            render_pass.set_index_buffer(buf.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..buf.index_count, 0, 0..1);
        }
    }
}

fn zip_vertices(positions: &[[f32; 2]], uvs: &[[f32; 2]]) -> Vec<Vertex> {
    positions
        .iter()
        .zip(uvs.iter())
        .map(|(pos, uv)| Vertex {
            position: *pos,
            // Live2D UV origin is bottom-left (OpenGL convention).
            // wgpu texture origin is top-left, so flip V.
            uv: [uv[0], 1.0 - uv[1]],
        })
        .collect()
}
