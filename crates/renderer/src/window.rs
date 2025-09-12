use std::sync::Arc;
use wgpu::rwh::{ HasWindowHandle, HasDisplayHandle };

pub struct WindowCtx<W: HasWindowHandle + HasDisplayHandle> {
    window: Arc<W>,
    width: u32,
    height: u32,
}

impl<W: HasWindowHandle + HasDisplayHandle> WindowCtx<W> {
    pub fn new(window: Arc<W>, width: u32, height: u32) -> Self {
        Self { window, width, height }
    }

    pub fn window(&self) -> &Arc<W> {
        &self.window
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}