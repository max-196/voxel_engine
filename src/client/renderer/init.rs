use {
    super::{Renderer, depth_texture::DepthTexture, err::RendererError},
    winit::{window::Window, dpi::PhysicalSize},
    pollster::block_on
};

impl Renderer {
    pub fn new(window: &Window) -> Result<Self, RendererError> {
        let size = window.inner_size();

        let instance = Self::get_instance();

        let surface = Self::get_surface(&instance, window);

        let adapter = block_on(Self::get_adapter(&instance, &surface))?;

        let (device, queue) = block_on(Self::get_device(&adapter))?;

        let config = Self::get_config(&adapter, &surface, size);
        surface.configure(&device, &config);

        let depth_texture = DepthTexture::new(&device, &config);

        Ok(Self {
            surface,
            device,
            queue,
            config,
            size,
            depth_texture,
        })
    }

    fn get_instance() -> wgpu::Instance {
        wgpu::Instance::new(wgpu::Backends::all())
    }

    fn get_surface(instance: &wgpu::Instance, window: &winit::window::Window) -> wgpu::Surface {
        unsafe { instance.create_surface(window) }
    }

    async fn get_adapter(instance: &wgpu::Instance, surface: &wgpu::Surface) -> Result<wgpu::Adapter, RendererError> {
        instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(surface),
                force_fallback_adapter: false,
            },
        ).await.ok_or(RendererError::AdapterNotFound)
    }

    async fn get_device(adapter: &wgpu::Adapter) -> Result<(wgpu::Device, wgpu::Queue), RendererError> {
        adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None, // Trace path
        ).await.map_err(RendererError::RequestDeviceError)
    }

    fn get_config(adapter: &wgpu::Adapter, surface: &wgpu::Surface, size: PhysicalSize<u32>) -> wgpu::SurfaceConfiguration {
        wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoNoVsync,
        }
    }
}