mod sky;

pub use sky::SkyPass;

use wgpu::{CommandEncoder, Device, TextureFormat, TextureView};

pub struct ClearPass;

impl ClearPass {
    pub fn queue(&mut self, cmd: &mut CommandEncoder, target_texture_view: &TextureView) {
        cmd.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target_texture_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
    }
}

pub struct RenderPasses {
    clear: ClearPass,
    sky: SkyPass,
}

impl RenderPasses {
    pub fn new(device: &Device, surface_format: TextureFormat) -> Self {
        Self {
            clear: ClearPass,
            sky: SkyPass::new(device, surface_format),
        }
    }

    pub fn queue_all(&mut self, cmd: &mut CommandEncoder, target_texture_view: &TextureView) {
        self.clear.queue(cmd, &target_texture_view);
        self.sky.queue(cmd, &target_texture_view);
    }
}
