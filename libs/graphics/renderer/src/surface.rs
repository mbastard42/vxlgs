use crate::{
    WindowCtx,
    GpuCtx,
};

use std::marker::PhantomData;
use wgpu::rwh::{ HasWindowHandle, HasDisplayHandle };

pub struct SurfaceCtx<W: HasWindowHandle + HasDisplayHandle + Sync + Send + 'static> {
    pub surface: wgpu::Surface<'static>,
    pub config: Option<wgpu::SurfaceConfiguration>,
    _marker: PhantomData<W>,
}

impl<W: HasWindowHandle + HasDisplayHandle + Sync + Send + 'static> SurfaceCtx<W> {
    pub fn new(window_ctx: &WindowCtx<W>, gpu: &GpuCtx<W>) -> anyhow::Result<Self> {
        let surface: wgpu::Surface<'_> = gpu.instance.create_surface(window_ctx.window().clone())?;
        let caps = surface.get_capabilities(&gpu.adapter);

        let format = caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: window_ctx.size().0,
            height: window_ctx.size().1,
            present_mode: caps.present_modes[0],
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&gpu.device, &config);

        Ok(Self { 
            surface, 
            config: Some(config), 
            _marker: PhantomData 
        })
    }

    pub fn resize(&mut self, gpu: &GpuCtx<W>, width: u32, height: u32) {
        if width == 0 || height == 0 {
            self.config = None;
            return;
        }

        if let Some(ref mut config) = self.config {
            config.width = width;
            config.height = height;
            self.surface.configure(&gpu.device, config);
        }
    }

    pub fn is_configured(&self) -> bool {
        self.config.is_some()
    }
}
