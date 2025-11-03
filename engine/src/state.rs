use std::sync::Arc;

use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode, window::Window};

use crate::{
    math::units::Pixels,
    sprite::{
        atlas::{Atlas, AtlasConfig, AtlasRegionDescriptor},
        SpriteId,
    },
    texture::Texture,
};

pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,

    is_surface_configured: bool,

    pub window: Arc<Window>,

    renderer: crate::renderer::Renderer2D,
    camera: crate::camera::Camera2D,

    atlas: Atlas,
}

impl State {
    pub async fn new(window: Arc<Window>) -> Result<Self, ()> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let diffuse_bytes = include_bytes!("../../assets/atlas.png");
        let diffuse_texture =
            Arc::from(Texture::from_bytes(&device, &queue, diffuse_bytes, "atlas.png").unwrap());

        let atlas = Atlas::new(
            diffuse_texture.clone(),
            [
                AtlasRegionDescriptor {
                    id: SpriteId(1),
                    coords: glam::UVec2::new(0, 0),
                },
                AtlasRegionDescriptor {
                    id: SpriteId(2),
                    coords: glam::UVec2::new(1, 0),
                },
                AtlasRegionDescriptor {
                    id: SpriteId(3),
                    coords: glam::UVec2::new(2, 0),
                },
                AtlasRegionDescriptor {
                    id: SpriteId(4),
                    coords: glam::UVec2::new(1, 1),
                },
            ],
            AtlasConfig {
                tile_size: [Pixels::new(16.0), Pixels::new(16.0)],
            },
        );

        let camera = crate::camera::Camera2D::new(crate::camera::Camera2DConfig {
            position: glam::Vec2::new(0.0, 0.0),
            zoom: 1.0,
            viewport_size: glam::Vec2::new(size.width as f32, size.height as f32),
        });

        let renderer = crate::renderer::Renderer2D::new(
            &device,
            crate::renderer::Renderer2DConfig {
                texture: &diffuse_texture,
                texture_format: config.format,

                camera: &camera,
            },
        );

        return Ok(Self {
            surface,
            queue,
            device,
            config,
            window,

            is_surface_configured: false,

            renderer,
            camera,

            atlas,
        });
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;

            self.camera.resize(width as f32, height as f32);
        }
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.window.request_redraw();

        if !self.is_surface_configured {
            return Ok(());
        }

        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.renderer
            .render(&self.device, &self.queue, &view, &self.camera);

        output.present();

        return Ok(());
    }

    pub fn handle_key(&self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        match (code, is_pressed) {
            (KeyCode::Escape, true) => event_loop.exit(),
            _ => {}
        }
    }

    pub fn handle_wheel(&mut self, delta: f32) {
        self.camera.zoom_by(delta);
    }
}
