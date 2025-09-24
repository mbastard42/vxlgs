use prelude::*;

use std::marker::PhantomData;
use wgpu::rwh::{ HasWindowHandle, HasDisplayHandle };

use crate::{ WindowCtx, GpuCtx, PipelineCtx, SurfaceCtx };

pub struct FrameCtx<W: HasWindowHandle + HasDisplayHandle + std::marker::Sync + std::marker::Send + 'static> {
    _marker: PhantomData<W>,
}

impl<W: HasWindowHandle + HasDisplayHandle + std::marker::Sync + std::marker::Send + 'static> FrameCtx<W> {
    pub fn draw(gpu: &GpuCtx<W>, surface: &mut SurfaceCtx<W>, pipe: &PipelineCtx<W>) -> Result<()> {
        if !surface.is_configured() {
            return Ok(());
        }

        let output: wgpu::SurfaceTexture = surface.surface.get_current_texture().corerr()?;
        let view: wgpu::TextureView = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder: wgpu::CommandEncoder = gpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("render_encoder"),
        });

        {
            let mut rp: wgpu::RenderPass<'_> = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            rp.set_pipeline(&pipe.render_pipeline);
            rp.draw(0..3, 0..1);
        }

        gpu.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }

    pub fn redraw(window_ctx: &WindowCtx<W>, gpu_ctx: &GpuCtx<W>, surface_ctx: &mut SurfaceCtx<W>, pipeline_ctx: &PipelineCtx<W>) {
        if let Err(e) = FrameCtx::draw(gpu_ctx, surface_ctx, pipeline_ctx) {
            match e {
                Error::External(wgpu::SurfaceError::Lost) | wgpu::SurfaceError::Outdated => {
                    let width: u32 = window_ctx.size().0;
                    let height: u32 = window_ctx.size().1;
                    surface_ctx.resize(gpu_ctx, width, height);
                }
                other => log::error!("render error: {other}"),
            }
        }
    }
}  
