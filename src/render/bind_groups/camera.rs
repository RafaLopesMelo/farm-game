use crate::world::camera::Camera;
use wgpu::util::DeviceExt;

pub struct CameraBindGroup {
    buffer: Option<wgpu::Buffer>,
    bg: Option<wgpu::BindGroup>,
    bg_layout: Option<wgpu::BindGroupLayout>,
}

impl CameraBindGroup {
    pub fn new() -> Self {
        return Self {
            buffer: None,
            bg: None,
            bg_layout: None,
        };
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        if self.bg.is_none() {
            panic!("bind group is empty, call `write` first");
        }

        return self.bg.as_ref().unwrap();
    }

    pub fn layout(&self) -> &wgpu::BindGroupLayout {
        if self.bg_layout.is_none() {
            panic!("bind group layout is empty, call `write` first");
        }

        return self.bg_layout.as_ref().unwrap();
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        if self.buffer.is_none() {
            panic!("bind group buffer is empty, call `write` first");
        }

        return self.buffer.as_ref().unwrap();
    }

    pub fn write(&mut self, camera: &Camera, device: &wgpu::Device, queue: &wgpu::Queue) {
        let already_bound = self.bg.is_some();

        if self.buffer.is_none() {
            self.build_buffer(camera, device);
        }

        if self.bg.is_none() || self.bg_layout.is_none() {
            self.build_bind_group(device);
        }

        if !already_bound {
            return;
        }

        queue.write_buffer(
            &self.buffer.as_ref().unwrap(),
            0,
            bytemuck::cast_slice(&[camera.to_array()]),
        );
    }

    fn build_buffer(&mut self, camera: &Camera, device: &wgpu::Device) {
        let content = &camera.to_array();

        let buffer_desc = wgpu::util::BufferInitDescriptor {
            label: Some("camera_buffer"),
            contents: bytemuck::cast_slice(content),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        };
        let buffer = device.create_buffer_init(&buffer_desc);

        self.buffer = Some(buffer);
    }

    fn build_bind_group(&mut self, device: &wgpu::Device) {
        let buffer = self.buffer.as_ref().unwrap();

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

        self.bg = Some(bind_group);
        self.bg_layout = Some(layout);
    }
}
