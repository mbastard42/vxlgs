use crate::GpuCtx;

use std::marker::PhantomData;
use wgpu::rwh::{ HasWindowHandle, HasDisplayHandle };
pub struct ResourceCtx<W: HasWindowHandle + HasDisplayHandle + std::marker::Sync + std::marker::Send> {
    pub storage_tex_view: wgpu::TextureView,
    pub camera_ubo: wgpu::Buffer,
    pub cs_bgl: wgpu::BindGroupLayout,
    pub cs_bind: wgpu::BindGroup,
    _marker: PhantomData<W>,
}

impl<W: HasWindowHandle + HasDisplayHandle + std::marker::Sync + std::marker::Send> ResourceCtx<W> {
    pub fn new(gpu: &GpuCtx<W>) -> Self {
        let tex = gpu.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("storage_tex"),
            size: wgpu::Extent3d { width: 30, height: 30, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let storage_tex_view: wgpu::TextureView = tex.create_view(&wgpu::TextureViewDescriptor::default());

        let camera_ubo = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("camera_ubo"),
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let cs_bgl: wgpu::BindGroupLayout = gpu.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("cs_bgl"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
            ],
        });

        let cs_bind: wgpu::BindGroup = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("cs_bind"),
            layout: &cs_bgl,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: camera_ubo.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&storage_tex_view) },
            ],
        });

        Self { storage_tex_view, camera_ubo, cs_bgl, cs_bind, _marker: PhantomData }
    }
}
