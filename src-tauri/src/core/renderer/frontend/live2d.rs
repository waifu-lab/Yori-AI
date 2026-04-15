use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use live2d_wgpu_renderer::{load_texture, Live2dModel, Live2dRenderer};
use parking_lot::Mutex;

use super::super::backend::RenderState;

pub struct Live2dFrontend {
    model: Mutex<Live2dModel>,
    renderer: Mutex<Live2dRenderer>,
}

impl Live2dFrontend {
    pub fn new(dir: PathBuf, state: &RenderState) -> Result<Self> {
        let device = state.device();
        let queue = state.queue();

        let model = Live2dModel::load(&dir)?;

        // Load textures listed in model3.json
        let textures: Vec<wgpu::Texture> = model
            .texture_paths
            .iter()
            .map(|p| load_texture(device, queue, p))
            .collect::<Result<_>>()?;
        let texture_refs: Vec<&wgpu::Texture> = textures.iter().collect();

        // Initial snapshot to size GPU buffers
        model.update();
        let snapshots = model.drawable_snapshots();

        let renderer = Live2dRenderer::new(
            device,
            queue,
            state.target_format(),
            &snapshots,
            &texture_refs,
        );

        Ok(Self {
            model: Mutex::new(model),
            renderer: Mutex::new(renderer),
        })
    }
}

impl super::Frontend for Live2dFrontend {
    fn render(&self, state: Arc<RenderState>) {
        // ── 1. advance model, snapshot drawable data ───────────────────────
        let model = self.model.lock();
        model.update();
        let snapshots = model.drawable_snapshots();
        drop(model);

        let device = state.device();
        let queue = state.queue();

        // ── 2. upload dirty buffers (before render pass) ───────────────────
        let renderer = self.renderer.lock();
        renderer.update(queue, &snapshots);

        // ── 3. record render commands ──────────────────────────────────────
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("live2d_encoder"),
        });

        // Mask pre-render: must happen before the main color pass.
        renderer.encode_mask_passes(&mut encoder, &snapshots);

        {
            let view = state.target_view();
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("live2d_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            renderer.render(&mut pass, &snapshots);
        }

        queue.submit(std::iter::once(encoder.finish()));
    }
}
