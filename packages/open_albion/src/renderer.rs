mod passes;

use self::passes::RenderPasses;
use derive_more::{Display, Error};
use wgpu::{
    CompositeAlphaMode, CreateSurfaceError, Device, DeviceDescriptor, Instance, InstanceDescriptor,
    PresentMode, Queue, RequestAdapterError, RequestAdapterOptions, RequestDeviceError, Surface,
    SurfaceConfiguration, SurfaceError, SurfaceTarget, SurfaceTexture, TextureFormat,
    TextureUsages,
};

pub struct Renderer<'target> {
    device: Device,
    queue: Queue,
    surface: Surface<'target>,
    surface_format: TextureFormat,
    passes: RenderPasses,
}

impl<'target> Renderer<'target> {
    pub async fn new(target: impl Into<SurfaceTarget<'target>>) -> Result<Self, NewRendererError> {
        use NewRendererError as E;

        let instance = Instance::new(&InstanceDescriptor::default());

        let adapter = instance
            .request_adapter(&RequestAdapterOptions::default())
            .await
            .map_err(E::RequestAdapter)?;

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor::default())
            .await
            .map_err(E::RequestDevice)?;

        let surface = instance.create_surface(target).map_err(E::CreateSurface)?;

        let surface_capabilities = surface.get_capabilities(&adapter);

        let &surface_format = surface_capabilities
            .formats
            .get(0)
            .unwrap_or(&TextureFormat::Rgba8UnormSrgb);

        let passes = RenderPasses::new(&device, surface_format);

        Ok(Self {
            surface,
            surface_format,
            device,
            queue,
            passes,
        })
    }

    pub fn resize_surface(&self, width: u32, height: u32) {
        self.surface.configure(
            &self.device,
            &SurfaceConfiguration {
                usage: TextureUsages::RENDER_ATTACHMENT,
                format: self.surface_format,
                view_formats: vec![self.surface_format.add_srgb_suffix()],
                alpha_mode: CompositeAlphaMode::Auto,
                width,
                height,
                desired_maximum_frame_latency: 2,
                present_mode: PresentMode::AutoVsync,
            },
        );
    }

    pub fn render(&mut self) -> Result<PrePresent, SurfaceError> {
        let surface_texture = self.surface.get_current_texture()?;

        let surface_texture_view =
            surface_texture
                .texture
                .create_view(&wgpu::TextureViewDescriptor {
                    format: Some(self.surface_format.add_srgb_suffix()),
                    ..Default::default()
                });

        let mut cmd = self.device.create_command_encoder(&Default::default());

        self.passes.queue_all(&mut cmd, &surface_texture_view);

        self.queue.submit([cmd.finish()]);

        Ok(PrePresent(surface_texture))
    }
}

pub struct PrePresent(SurfaceTexture);

impl PrePresent {
    pub fn present(self) {
        self.0.present();
    }
}

#[derive(Error, Display, Debug)]
pub enum NewRendererError {
    RequestAdapter(RequestAdapterError),
    RequestDevice(RequestDeviceError),
    CreateSurface(CreateSurfaceError),
}
