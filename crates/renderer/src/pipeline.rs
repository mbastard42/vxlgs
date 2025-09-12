use std::marker::PhantomData;
use wgpu::rwh::{ HasWindowHandle, HasDisplayHandle };

use crate::{
    GpuCtx,
    SurfaceCtx,
};

pub struct PipelineCtx<W: HasWindowHandle + HasDisplayHandle + std::marker::Sync + std::marker::Send + 'static> {
    pub shader: wgpu::ShaderModule,
    pub pipeline_layout: wgpu::PipelineLayout,
    pub render_pipeline: wgpu::RenderPipeline,
    _marker: PhantomData<W>,
}

impl<W: HasWindowHandle + HasDisplayHandle + std::marker::Sync + std::marker::Send + 'static> PipelineCtx<W> {
    pub fn new(gpu: &GpuCtx<W>, surface: &SurfaceCtx<W>) -> Self {
        let shader: wgpu::ShaderModule = gpu.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/src/shader.wgsl").into()),
        });

        let pipeline_layout: wgpu::PipelineLayout = gpu.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("pipeline_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline: wgpu::RenderPipeline = gpu.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("render_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface.config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self { shader, pipeline_layout, render_pipeline, _marker: PhantomData }
    }
}
