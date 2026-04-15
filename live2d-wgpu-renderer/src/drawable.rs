use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

/// Interleaved vertex: position (x,y) + UV (u,v)
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub uv: [f32; 2],
}

impl Vertex {
    pub const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[
            wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x2,
            },
            wgpu::VertexAttribute {
                offset: 8,
                shader_location: 1,
                format: wgpu::VertexFormat::Float32x2,
            },
        ],
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Normal,
    Additive,
    Multiplicative,
}

pub struct DrawableBuffers {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,

    pub opacity_buffer: wgpu::Buffer,
    pub opacity_bind_group: wgpu::BindGroup,
}

impl DrawableBuffers {
    pub fn new(
        device: &wgpu::Device,
        vertices: &[Vertex],
        indices: &[u16],
        opacity_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("live2d_vertex"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("live2d_index"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        let opacity_data: [f32; 4] = [1.0, 0.0, 0.0, 0.0];
        let opacity_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("live2d_opacity"),
            contents: bytemuck::cast_slice(&opacity_data),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let opacity_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("live2d_opacity_bg"),
            layout: opacity_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: opacity_buffer.as_entire_binding(),
            }],
        });
        Self {
            vertex_buffer,
            index_buffer,
            index_count: indices.len() as u32,
            opacity_buffer,
            opacity_bind_group,
        }
    }

    pub fn update_vertices(&self, queue: &wgpu::Queue, vertices: &[Vertex]) {
        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(vertices));
    }

    /// Upload per-drawable shader params.
    /// `u_params.x` = opacity, `u_params.y` = is_inverted_mask (0.0 or 1.0).
    pub fn update_params(&self, queue: &wgpu::Queue, opacity: f32, is_inverted_mask: bool) {
        let inv = if is_inverted_mask { 1.0_f32 } else { 0.0 };
        let data: [f32; 4] = [opacity, inv, 0.0, 0.0];
        queue.write_buffer(&self.opacity_buffer, 0, bytemuck::cast_slice(&data));
    }
}
