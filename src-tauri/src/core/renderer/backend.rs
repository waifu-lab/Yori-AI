use std::sync::{Arc, OnceLock};

pub static RENDER_BACKEND: OnceLock<Arc<RenderBackend>> = OnceLock::new();
pub static SHAERD_STATE: OnceLock<Arc<RenderState>> = OnceLock::new();

pub struct RenderBackend {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

pub struct RenderState {
    backend: Arc<RenderBackend>,
    texture: wgpu::Texture,
    depth_texture: Option<wgpu::Texture>,
    size: dpi::PhysicalSize<u32>,
}

pub struct RenderHandle {
    state: RenderState,
    window: Arc<tauri::WebviewWindow>,
    surface: wgpu::Surface<'static>,
}

impl RenderBackend {
    pub async fn new() -> anyhow::Result<Self> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            backend_options: wgpu::BackendOptions::default(),
            memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
            flags: wgpu::InstanceFlags::empty(),
        });
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: None
        }).await?;
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("YoriAI_device"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::defaults(),
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
            memory_hints: wgpu::MemoryHints::default(),
            trace: wgpu::Trace::Off
        }).await?;
        Ok(Self {
            instance,
            adapter,
            device,
            queue
        })
    }

    pub fn d2_state(self: Arc<Self>, size: dpi::PhysicalSize<u32>) -> RenderState {
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("YoriAI_2d_texture"),
            size: wgpu::Extent3d {
                width: size.width,
                height: size.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        RenderState {
            backend: self.clone(),
            texture,
            depth_texture: None,
            size
        }
    }

    pub fn render_handle(self: Arc<Self>, window: Arc<tauri::WebviewWindow>) -> anyhow::Result<RenderHandle> {
        let size = window.inner_size()?;
        let state = self.clone().d2_state(size);
        let surface = self.clone().instance.create_surface(window.clone())?;
        Ok(RenderHandle {
            state,
            window,
            surface,
        })
    }
}