use glam::Mat4;

const MIN_ZOOM: f32 = 0.5;
const MAX_ZOOM: f32 = 2.0;
const ZOOM_SPEED: f32 = 0.1;

pub struct Camera2D {
    position: glam::Vec2,
    zoom: f32,
    viewport_size: glam::Vec2,
}

pub struct Camera2DConfig {
    pub position: glam::Vec2,
    pub zoom: f32,
    pub viewport_size: glam::Vec2,
}

impl Camera2D {
    pub fn new(config: Camera2DConfig) -> Self {
        return Self {
            position: config.position,
            zoom: config.zoom,
            viewport_size: config.viewport_size,
        };
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.viewport_size = glam::Vec2::new(width, height);
    }

    pub fn zoom_by(&mut self, delta: f32) {
        let factor = (1.0 + ZOOM_SPEED).powf(delta);
        self.set_zoom(self.zoom * factor);
    }

    fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.clamp(MIN_ZOOM, MAX_ZOOM);
    }

    pub fn translate(&mut self, delta: glam::Vec2) {
        self.set_position(self.position + delta);
    }

    fn set_position(&mut self, position: glam::Vec2) {
        self.position = position;
    }

    pub fn view_matrix(&self) -> glam::Mat4 {
        return glam::Mat4::from_translation((-self.position).extend(0.0));
    }

    pub fn projection_matrix(&self) -> Mat4 {
        let half_width = 0.5 * self.viewport_size.x / self.zoom;
        let half_height = 0.5 * self.viewport_size.y / self.zoom;

        let near = -1.0;
        let far = 1.0;

        return Mat4::orthographic_rh_gl(
            -half_width,
            half_width,
            -half_height,
            half_height,
            near,
            far,
        );
    }

    pub fn view_projection(&self) -> Mat4 {
        return self.projection_matrix() * self.view_matrix();
    }

    pub fn world_to_screen(&self, world: glam::Vec2) -> glam::Vec2 {
        let world_h = glam::Vec3::new(world.x, world.y, 0.0);

        let clip = self.view_projection().project_point3(world_h);

        let ndc_to_unit = |value: f32| value * 0.5 + 0.5;

        let x = ndc_to_unit(clip.x) * self.viewport_size.x;
        let y = (1.0 - ndc_to_unit(clip.y)) * self.viewport_size.y;

        return glam::Vec2::new(x, y);
    }

    pub fn screen_to_world(&self, screen: glam::Vec2) -> glam::Vec2 {
        let ndc_x = (screen.x / self.viewport_size.x) * 2.0 - 1.0;
        let ndc_y = 1.0 - (screen.y / self.viewport_size.y) * 2.0;
        let ndc = glam::Vec3::new(ndc_x, ndc_y, 0.0);

        let world = self.view_projection().inverse().project_point3(ndc);

        world.truncate()
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn from_camera(camera: &Camera2D) -> Self {
        return Self {
            view_proj: camera.view_projection().to_cols_array_2d(),
        };
    }
}
