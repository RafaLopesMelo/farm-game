use crate::world::camera::Camera;
use wgpu::util::DeviceExt;

pub struct CameraBindGroup {
    buffer: wgpu::Buffer,
    bg: wgpu::BindGroup,
    layout: wgpu::BindGroupLayout,
}

impl CameraBindGroup {
    pub fn new(camera: &Camera, device: &wgpu::Device) -> Self {
        let buffer = Self::build_buffer(camera, device);
        let (bg, layout) = Self::build_bind_group(&buffer, device);

        return Self { buffer, bg, layout };
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        return &self.bg;
    }

    pub fn layout(&self) -> &wgpu::BindGroupLayout {
        return &self.layout;
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        return &self.buffer;
    }

    pub fn write(&mut self, camera: &Camera, queue: &wgpu::Queue) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[camera.to_array()]));
    }

    fn build_buffer(camera: &Camera, device: &wgpu::Device) -> wgpu::Buffer {
        let content = &camera.to_array();

        let buffer_desc = wgpu::util::BufferInitDescriptor {
            label: Some("camera_buffer"),
            contents: bytemuck::cast_slice(content),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        };
        let buffer = device.create_buffer_init(&buffer_desc);

        return buffer;
    }

    fn build_bind_group(
        buffer: &wgpu::Buffer,
        device: &wgpu::Device,
    ) -> (wgpu::BindGroup, wgpu::BindGroupLayout) {
        let layout_desc = wgpu::BindGroupLayoutDescriptor {
            label: Some("camera_bind_group_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        };
        let layout = device.create_bind_group_layout(&layout_desc);

        let bind_group_desc = wgpu::BindGroupDescriptor {
            label: Some("camera_bind_group"),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        };
        let bind_group = device.create_bind_group(&bind_group_desc);

        return (bind_group, layout);
    }
}
