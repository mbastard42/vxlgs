use crate::{
    WindowCtx,
    GpuCtx,
};

use std::marker::PhantomData;
use wgpu::rwh::{ HasWindowHandle, HasDisplayHandle };

pub struct SurfaceCtx<W: HasWindowHandle + HasDisplayHandle + std::marker::Sync + std::marker::Send + 'static> {
    pub surface: wgpu::Surface<'static>,
    pub config: wgpu::SurfaceConfiguration,
    pub is_configured: bool,
    _marker: PhantomData<W>,
}

impl<W: HasWindowHandle + HasDisplayHandle + std::marker::Sync + std::marker::Send + 'static> SurfaceCtx<W> {
    pub fn new(window_ctx: &WindowCtx<W>, gpu: &GpuCtx<W>) -> anyhow::Result<Self> {
        let surface: wgpu::Surface<'_> = gpu.instance.create_surface(window_ctx.window().clone())?;
        let caps: wgpu::SurfaceCapabilities = surface.get_capabilities(&gpu.adapter);

        let format: wgpu::TextureFormat = caps
            .formats
            .iter()
            .copied()
            .find(|f: &wgpu::TextureFormat| f.is_srgb())
            .unwrap_or(caps.formats[0]);

        let config: wgpu::wgt::SurfaceConfiguration<Vec<wgpu::TextureFormat>> = wgpu::SurfaceConfiguration {
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
        let is_configured = true;

        Ok(Self { surface, config, is_configured, _marker: PhantomData })
    }

    pub fn resize(&mut self, gpu: &GpuCtx<W>, width: u32, height: u32) {
        if width == 0 || height == 0 {
            self.is_configured = false;
            return;
        }
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&gpu.device, &self.config);
        self.is_configured = true;
        
    }
}
