use std::sync::Arc;
use log::{error, info};
use wgpu::{SurfaceError, TextureFormat};
use winit::{application::ApplicationHandler,event::WindowEvent,event_loop::{ActiveEventLoop,ControlFlow,EventLoop},window::{Window,WindowId}};

#[cfg(target_os = "android")]
use winit::platform::android::{activity::AndroidApp,EventLoopBuilderExtAndroid};

struct GpuState {
    surface: wgpu::Surface<'static>,
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
}

impl GpuState {
    async fn new(window: Arc<Window>) -> Result<Self, String> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN,
            ..Default::default()
        });
        let surface = instance.create_surface(window.clone())
            .map_err(|e| format!("surface creation failed: {e}"))?;
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.map_err(|e| format!("Vulkan adapter request failed: {e}"))?;

        let info = adapter.get_info();
        info!("Vulkan adapter: name={} vendor={} device={} driver={} driver_info={} backend={:?}",
            info.name, info.vendor, info.device, info.driver, info.driver_info, info.backend);
        info!("Adapter features: {:?}", adapter.features());
        info!("Adapter limits: {:?}", adapter.limits());

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("rusty-android-vulkan-poc-device"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::downlevel_defaults(),
            memory_hints: wgpu::MemoryHints::Performance,
            trace: wgpu::Trace::Off,
        }).await.map_err(|e| format!("Vulkan device request failed: {e}"))?;

        let caps = surface.get_capabilities(&adapter);
        if caps.formats.is_empty() || caps.alpha_modes.is_empty() {
            return Err("surface reported no usable formats or alpha modes".into());
        }
        let format = caps.formats.iter().copied().find(TextureFormat::is_srgb)
            .unwrap_or(caps.formats[0]);
        let present_mode = if caps.present_modes.contains(&wgpu::PresentMode::Fifo) {
            wgpu::PresentMode::Fifo
        } else {
            *caps.present_modes.first().ok_or("surface reported no present modes")?
        };
        let size = window.inner_size();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        info!("Surface: format={:?}, present_mode={:?}, alpha_mode={:?}, size={}x{}",
            config.format, config.present_mode, config.alpha_mode, config.width, config.height);
        surface.configure(&device, &config);
        Ok(Self { surface, window, device, queue, config })
    }

    fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 { return; }
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
        info!("Surface reconfigured: {width}x{height}");
    }

    fn render(&mut self) -> Result<(), SurfaceError> {
        let frame = self.surface.get_current_texture()?;
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("rusty-vulkan-poc-clear"),
        });
        {
            let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("rusty-vulkan-poc-clear-pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.025, g: 0.12, b: 0.20, a: 1.0 }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }
        self.queue.submit(Some(encoder.finish()));
        frame.present();
        Ok(())
    }
}

#[derive(Default)]
struct App { gpu: Option<GpuState> }

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::Poll);
        if self.gpu.is_some() { return; }
        info!("Lifecycle: Resumed; creating window and Vulkan surface");
        let window = match event_loop.create_window(Window::default_attributes()
            .with_title("Rusty Survival — Vulkan POC")) {
            Ok(w) => Arc::new(w),
            Err(e) => { error!("Window creation failed: {e}"); event_loop.exit(); return; }
        };
        match pollster::block_on(GpuState::new(window)) {
            Ok(gpu) => { info!("Vulkan surface initialized successfully"); self.gpu = Some(gpu); }
            Err(e) => { error!("Vulkan initialization failed: {e}"); event_loop.exit(); }
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        info!("Lifecycle: Suspended; releasing surface-dependent resources");
        self.gpu = None;
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let Some(gpu) = self.gpu.as_mut() else { return; };
        if gpu.window.id() != id { return; }
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => gpu.resize(size.width, size.height),
            WindowEvent::RedrawRequested => match gpu.render() {
                Ok(()) => {}
                Err(SurfaceError::Lost | SurfaceError::Outdated) => {
                    let size = gpu.window.inner_size();
                    gpu.resize(size.width, size.height);
                }
                Err(SurfaceError::Timeout) => info!("Surface frame timed out; skipping frame"),
                Err(SurfaceError::OutOfMemory) => { error!("GPU ran out of memory; exiting"); event_loop.exit(); }
                Err(e) => { error!("Surface rendering failed: {e}"); event_loop.exit(); }
            },
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(gpu) = &self.gpu { gpu.window.request_redraw(); }
    }
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(android_app: AndroidApp) {
    android_logger::init_once(android_logger::Config::default()
        .with_max_level(log::LevelFilter::Info));
    info!("Rusty Survival Android Vulkan POC starting");
    let mut builder = EventLoop::<()>::builder();
    builder.with_android_app(android_app);
    let event_loop = match builder.build() {
        Ok(e) => e,
        Err(e) => { error!("Event loop creation failed: {e}"); return; }
    };
    let mut app = App::default();
    if let Err(e) = event_loop.run_app(&mut app) {
        error!("Event loop exited with error: {e}");
    }
}
