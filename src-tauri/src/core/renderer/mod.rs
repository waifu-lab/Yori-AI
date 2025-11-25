use std::sync::Arc;

pub struct Renderer {
    state: State
}

struct State {
    window_label: Option<String>,
    surface: Option<wgpu::Surface<'static>>,
    texture: Option<wgpu::Texture>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: Option<wgpu::SurfaceConfiguration>,
    canvas_size: dpi::PhysicalSize<u32>,
    size_changed: bool
}

impl Renderer {
    pub async fn new(mut size: dpi::PhysicalSize<u32>) -> Self {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: None
        }).await.unwrap();
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("lumina-renderer-device"),
            required_features: wgpu::Features::empty(),
            ..Default::default()
        }, None).await.unwrap();

        size.width = size.width.max(1);
        size.height = size.height.max(1);
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("lumina-renderer-texture"),
            size: wgpu::Extent3d { width: size.width, height: size.height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        Self {
            state: State {
                window_label: None,
                surface: None,
                texture: Some(texture),
                device,
                queue,
                config: None,
                canvas_size: size,
                size_changed: false
            }
        }
    }

    pub async fn new_with_window(window: Arc<tauri::WebviewWindow>) -> Self {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface)
        }).await.unwrap();
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("lumina-renderer-device"),
            required_features: wgpu::Features::empty(),
            ..Default::default()
        }, None).await.unwrap();

        let caps = surface.get_capabilities(&adapter);
        let format = caps.formats[0];
        let mut size = window.inner_size().unwrap();
        size.width = size.width.max(1);
        size.height = size.height.max(1);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2
        };

        surface.configure(&device, &config);
        Self {
            state: State {
                window_label: Some(window.label().to_string()),
                surface: Some(surface),
                texture: None,
                device,
                queue,
                config: Some(config),
                canvas_size: size,
                size_changed: false
            }
        }
    }
}