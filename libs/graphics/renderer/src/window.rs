use std::sync::Arc;
use wgpu::rwh::{ HasWindowHandle, HasDisplayHandle };

use crate::{ SurfaceCtx, GpuCtx };

pub struct WindowCtx<W: HasWindowHandle + HasDisplayHandle + std::marker::Sync + std::marker::Send + 'static> {
    window: Arc<W>,
    width: u32,
    height: u32,
}

impl<W: HasWindowHandle + HasDisplayHandle + std::marker::Sync + std::marker::Send + 'static> WindowCtx<W> {
    pub fn new(window: Arc<W>, width: u32, height: u32) -> Self {
        Self { window, width, height }
    }

    pub fn window(&self) -> &Arc<W> {
        &self.window
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn resize(&mut self, gpu_ctx: &GpuCtx<W>, surface_ctx: &mut SurfaceCtx<W>, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        surface_ctx.resize(&gpu_ctx, width, height);
    }
}