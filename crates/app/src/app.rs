use std::sync::Arc;
use renderer::{ WindowCtx, GpuCtx, SurfaceCtx, ResourceCtx, PipelineCtx, FrameCtx };
use winit::{ application::ApplicationHandler, event::*, event_loop::{ActiveEventLoop, EventLoop}, window::WindowId };

type Window = winit::window::Window;

struct App {
    window_ctx: Option<WindowCtx<Window>>,
    gpu_ctx: Option<GpuCtx<Window>>,
    surface_ctx: Option<SurfaceCtx<Window>>,
    resource_ctx: Option<ResourceCtx<Window>>,
    pipeline_ctx: Option<PipelineCtx<Window>>,
}

impl App {
    pub fn new() -> Self {
        Self { window_ctx: None, gpu_ctx: None, surface_ctx: None, resource_ctx: None, pipeline_ctx: None }
    }
}

impl ApplicationHandler<()> for App {

    fn resumed(&mut self, el: &ActiveEventLoop) {

        if self.window_ctx.is_none() {
            let window: Arc<Window> = Arc::new(el.create_window(Window::default_attributes()).unwrap());
            let width: u32 = window.inner_size().width;
            let height: u32 = window.inner_size().height;
            self.window_ctx = Some(WindowCtx::new(window, width, height));
        }
        let window_ctx: &WindowCtx<Window> = self.window_ctx.as_ref().unwrap();

        if self.gpu_ctx.is_none() {
            self.gpu_ctx = Some(pollster::block_on(GpuCtx::new(window_ctx)).unwrap());
        }
        let gpu_ctx: &GpuCtx<Window> = self.gpu_ctx.as_ref().unwrap();

        if self.surface_ctx.is_none() {
            self.surface_ctx = Some(SurfaceCtx::new(window_ctx, gpu_ctx).unwrap());
        }
        let surface_ctx: &mut SurfaceCtx<Window> = self.surface_ctx.as_mut().unwrap();

        if self.resource_ctx.is_none() {
            self.resource_ctx = Some(ResourceCtx::new(gpu_ctx));
        }

        if self.pipeline_ctx.is_none() {
            self.pipeline_ctx = Some(PipelineCtx::new(gpu_ctx, surface_ctx));
        }

        window_ctx.window().request_redraw();
    }

    fn window_event(&mut self, el: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let (Some(window_ctx), Some(gpu_ctx), Some(surface_ctx), Some(pipeline_ctx)) =
            (self.window_ctx.as_mut(), self.gpu_ctx.as_ref(), self.surface_ctx.as_mut(), self.pipeline_ctx.as_ref())
        else { return; };

        match event {
            WindowEvent::CloseRequested => el.exit(),
            WindowEvent::Resized(size) => window_ctx.resize(gpu_ctx, surface_ctx, size.width, size.height),
            WindowEvent::RedrawRequested => {
                FrameCtx::redraw(window_ctx, gpu_ctx, surface_ctx, pipeline_ctx);
                window_ctx.window().request_redraw();
            }
            _ => {}
        }
    }
}

pub fn run() -> anyhow::Result<()> {
    
    let event_loop: EventLoop<()> = EventLoop::with_user_event().build()?;
    let mut app: App = App::new();
    event_loop.run_app(&mut app)?;
    Ok(())
}