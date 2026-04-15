use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;

use anyhow::{Context, Result};
use live2d_wgpu_renderer::{Expression, Live2dModel, Live2dRenderer, load_texture};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowAttributes, WindowId};

struct GpuState {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    model: Live2dModel,
    renderer: Live2dRenderer,
    start_time: Instant,
    debug_printed: bool,
    expressions: Vec<Expression>,
    active_expression: Option<usize>,
    // viewport state
    canvas_size: (f32, f32),
    zoom: f32,
    pan: [f32; 2],
    drag: Option<[f32; 2]>, // cursor position when drag started
    pan_at_drag: [f32; 2],  // pan value when drag started
}

impl GpuState {
    async fn new(window: Arc<Window>, model_dir: &Path) -> Result<Self> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let surface = instance
            .create_surface(window.clone())
            .context("failed to create WGPU surface")?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .context("no suitable GPU adapter found")?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("live2d-device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                memory_hints: wgpu::MemoryHints::Performance,
                trace: wgpu::Trace::Off,
            })
            .await
            .context("failed to create device")?;

        let caps = surface.get_capabilities(&adapter);
        let format = caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .or_else(|| caps.formats.first().copied())
            .context("surface reports no supported formats")?;

        let size = window.inner_size();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width.max(1),
            height: size.height.max(1),
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let model = Live2dModel::load(model_dir)
            .with_context(|| format!("failed to load model from {}", model_dir.display()))?;

        let textures = model
            .texture_paths
            .iter()
            .map(|path| load_texture(&device, &queue, path))
            .collect::<Result<Vec<_>>>()
            .context("failed to load model textures")?;

        // Run one update so vertex positions are valid before building GPU buffers.
        model.update();

        let texture_refs = textures.iter().collect::<Vec<_>>();
        let snapshots = model.drawable_snapshots();
        let masked_count = snapshots.iter().filter(|s| !s.masks.is_empty()).count();
        let mask_src_invisible = snapshots
            .iter()
            .enumerate()
            .filter(|(i, s)| {
                s.masks.is_empty()
                    && snapshots.iter().any(|ms| ms.masks.contains(i))
                    && !s.is_visible
            })
            .count();
        eprintln!(
            "live2d drawables: total={}, masked={}, invisible_mask_sources={}",
            snapshots.len(),
            masked_count,
            mask_src_invisible
        );
        let renderer = Live2dRenderer::new(&device, &queue, format, &snapshots, &texture_refs);

        // Set the initial MVP to respect the model's canvas aspect ratio.
        let (cw, ch) = model.canvas_size_units();
        let sz = window.inner_size();
        renderer.set_viewport(&queue, cw, ch, sz.width.max(1), sz.height.max(1), 1.0, [0.0, 0.0]);

        // Load expressions from the model directory.
        let expressions = model.load_expressions();
        eprintln!("loaded {} expression(s):", expressions.len());
        for (i, e) in expressions.iter().enumerate() {
            eprintln!("  [{}] {}", i + 1, e.name);
        }
        if !expressions.is_empty() {
            eprintln!("press 1-{} to switch, 0 to reset", expressions.len());
        }

        Ok(Self {
            window,
            surface,
            device,
            queue,
            config,
            model,
            renderer,
            start_time: Instant::now(),
            debug_printed: false,
            expressions,
            active_expression: None,
            canvas_size: (cw, ch),
            zoom: 1.0,
            pan: [0.0, 0.0],
            drag: None,
            pan_at_drag: [0.0, 0.0],
        })
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 {
            return;
        }
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);

        self.update_mvp();
    }

    fn update_mvp(&self) {
        let (cw, ch) = self.canvas_size;
        let sz = self.window.inner_size();
        self.renderer.set_viewport(
            &self.queue,
            cw, ch,
            sz.width.max(1), sz.height.max(1),
            self.zoom,
            self.pan,
        );
    }

    fn set_expression(&mut self, idx: Option<usize>) {
        self.active_expression = idx;
        self.model.clear_expression();
        if let Some(i) = idx {
            if let Some(expr) = self.expressions.get(i) {
                eprintln!("expression → {}", expr.name);
                self.model.apply_expression(expr);
            }
        } else {
            eprintln!("expression → (none)");
        }
    }

    fn render(&mut self) -> Result<()> {
        let t = self.start_time.elapsed().as_secs_f32();
        self.model
            .set_parameter("ParamAngleX", (t * 1.2).sin() * 15.0);
        self.model
            .set_parameter("ParamBodyAngleX", (t * 0.9).sin() * 8.0);

        self.model.update();
        let snapshots = self.model.drawable_snapshots();
        self.renderer.update(&self.queue, &snapshots);

        // One-time render-order diagnostic: show how masked vs unmasked
        // drawables are interleaved so we can spot any face-covering-features issue.
        if !self.debug_printed {
            self.debug_printed = true;
            let mut order: Vec<usize> = (0..snapshots.len()).collect();
            order.sort_unstable_by_key(|&i| snapshots[i].render_order);

            // Find the render_order range of masked drawables
            let masked_orders: Vec<i32> = order
                .iter()
                .filter(|&&i| !snapshots[i].masks.is_empty())
                .map(|&i| snapshots[i].render_order)
                .collect();
            let (masked_min, masked_max) = masked_orders
                .iter()
                .fold((i32::MAX, i32::MIN), |(lo, hi), &v| (lo.min(v), hi.max(v)));

            eprintln!(
                "masked drawable render_order range: [{masked_min}, {masked_max}]"
            );

            // Print any unmasked drawable whose render_order falls INSIDE or ABOVE
            // the masked range (candidates that could cover face features).
            eprintln!("unmasked drawables with render_order >= {masked_min}:");
            for &i in &order {
                let s = &snapshots[i];
                if s.masks.is_empty() && s.is_visible && s.render_order >= masked_min {
                    eprintln!(
                        "  drawable[{i:3}] order={:3} tex={} opacity={:.2}",
                        s.render_order, s.texture_index, s.opacity
                    );
                }
            }
        }

        let frame = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
            Err(wgpu::SurfaceError::Timeout) => return Ok(()),
            Err(err) => return Err(anyhow::anyhow!("surface acquire failed: {err}")),
        };

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("live2d-encoder"),
            });

        // Must happen before the main render pass.
        self.renderer.encode_mask_passes(&mut encoder, &snapshots);

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("live2d-pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.08,
                            g: 0.10,
                            b: 0.13,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.renderer.render(&mut pass, &snapshots);
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
        Ok(())
    }
}

struct App {
    model_dir: PathBuf,
    state: Option<GpuState>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_some() {
            return;
        }

        let attrs = WindowAttributes::default()
            .with_title("Live2D WGPU Renderer")
            .with_inner_size(PhysicalSize::new(1280, 720));
        let window = match event_loop.create_window(attrs) {
            Ok(w) => Arc::new(w),
            Err(err) => {
                eprintln!("failed to create window: {err}");
                event_loop.exit();
                return;
            }
        };

        match pollster::block_on(GpuState::new(window, &self.model_dir)) {
            Ok(state) => {
                state.window.request_redraw();
                self.state = Some(state);
            }
            Err(err) => {
                eprintln!("initialization failed: {err:#}");
                event_loop.exit();
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(state) = self.state.as_mut() else {
            return;
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size),
            // ── zoom ──────────────────────────────────────────────────────
            WindowEvent::MouseWheel { delta, .. } => {
                let scroll = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y,
                    MouseScrollDelta::PixelDelta(pos) => pos.y as f32 / 40.0,
                };
                state.zoom = (state.zoom * 1.1_f32.powf(scroll)).clamp(0.1, 20.0);
                state.update_mvp();
            }
            // ── drag ──────────────────────────────────────────────────────
            WindowEvent::MouseInput { button: MouseButton::Left, state: btn_state, .. } => {
                if btn_state == ElementState::Pressed {
                    state.pan_at_drag = state.pan;
                    // cursor position stored in CursorMoved
                    state.drag = state.drag.or(Some([0.0, 0.0]));
                } else {
                    state.drag = None;
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let cur = [position.x as f32, position.y as f32];
                if let Some(ref mut start) = state.drag {
                    if *start == [0.0, 0.0] {
                        // first move after press — record start position
                        *start = cur;
                    } else {
                        let sz = state.window.inner_size();
                        let dx = (cur[0] - start[0]) * 2.0 / sz.width as f32;
                        let dy = -(cur[1] - start[1]) * 2.0 / sz.height as f32;
                        state.pan = [
                            state.pan_at_drag[0] + dx,
                            state.pan_at_drag[1] + dy,
                        ];
                        state.update_mvp();
                    }
                }
            }
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    physical_key: PhysicalKey::Code(code),
                    state: ElementState::Pressed,
                    ..
                },
                ..
            } => match code {
                KeyCode::Digit0 => state.set_expression(None),
                KeyCode::Digit1 => state.set_expression(Some(0)),
                KeyCode::Digit2 => state.set_expression(Some(1)),
                KeyCode::Digit3 => state.set_expression(Some(2)),
                KeyCode::Digit4 => state.set_expression(Some(3)),
                _ => {}
            },
            WindowEvent::ScaleFactorChanged { .. } => {
                state.resize(state.window.inner_size());
            }
            WindowEvent::RedrawRequested => {
                if let Err(err) = state.render() {
                    eprintln!("render error: {err:#}");
                    event_loop.exit();
                } else {
                    state.window.request_redraw();
                }
            }
            _ => {}
        }
    }
}

fn main() -> Result<()> {
    let model_dir = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .context("usage: cargo run --example window -- <model-directory>")?;

    let event_loop = EventLoop::new()?;
    let mut app = App {
        model_dir,
        state: None,
    };
    event_loop.run_app(&mut app)?;
    Ok(())
}