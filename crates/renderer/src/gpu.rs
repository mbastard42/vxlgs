use crate::WindowCtx;

use std::marker::PhantomData;
use wgpu::rwh::{ HasWindowHandle, HasDisplayHandle };

pub struct GpuCtx<W: HasWindowHandle + HasDisplayHandle + std::marker::Sync + std::marker::Send> {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    _marker: PhantomData<W>,
}

impl<W: HasWindowHandle + HasDisplayHandle + std::marker::Sync + std::marker::Send> GpuCtx<W> {
    pub async fn new(window_ctx: &WindowCtx<W>) -> anyhow::Result<Self> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let tmp_surface: wgpu::Surface<'_> = instance.create_surface(window_ctx.window().clone())?;

        let adapter: wgpu::Adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&tmp_surface),
                force_fallback_adapter: false,
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            }).await?;

        Ok(Self { instance, adapter, device, queue, _marker: PhantomData })
    }
}