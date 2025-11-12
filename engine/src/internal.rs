use std::sync::Arc;

use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode, window::Window};

use crate::{
    assets::{texture::Texture, AssetId, AssetsRegistry},
    ecs::{scheduler::System, ECS},
    render::{
        self,
        renderer::{Renderer2D, Renderer2DConfig},
        texture::GpuTextureManager,
    },
};

pub struct Internal {
    is_surface_configured: bool,

    pub window: Arc<Window>,

    texture_manager: render::texture::GpuTextureManager,
    assets_registry: AssetsRegistry,
    renderer: Renderer2D,
    camera: crate::camera::Camera2D,

    ecs: ECS,
}

impl Internal {
    pub async fn new(window: Arc<Window>) -> Result<Self, ()> {
        let size = window.inner_size();
        let camera = crate::camera::Camera2D::new(crate::camera::Camera2DConfig {
            position: glam::Vec2::new(0.0, 0.0),
            zoom: 1.0,
            viewport_size: glam::Vec2::new(size.width as f32, size.height as f32),
        });

        let renderer = Renderer2D::new(window.clone(), Renderer2DConfig { camera: &camera }).await;

        return Ok(Self {
            assets_registry: AssetsRegistry::new(),
            texture_manager: GpuTextureManager::new(),

            window,

            is_surface_configured: false,

            renderer,
            camera,

            ecs: ECS::new(),
        });
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.renderer.resize(width, height);
            self.camera.resize(width as f32, height as f32);

            self.is_surface_configured = true;
        }
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.window.request_redraw();
        return self.renderer.render(&self.camera);
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

    pub fn load_texture(&mut self, path: &str) -> AssetId {
        let buffer = image::io::Reader::open(path)
            .expect("Could not load texture image")
            .decode()
            .expect("Could not decode texture image")
            .into_rgba8();
        let texture = Texture::new();

        let id = self.assets_registry.insert_texture(texture);
        self.texture_manager
            .load(id, &self.renderer.device(), &self.renderer.queue(), &buffer);

        return id;
    }

    pub fn add_system<S>(&mut self, system: S)
    where
        S: System + 'static,
    {
        self.ecs.add_system(system);
    }

    pub fn run_systems(&mut self, dt: std::time::Duration) {
        self.ecs.run_systems(dt);
    }
}
