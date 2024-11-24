use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct ScreenUniform {
    size: [u32; 2],
}

pub struct ScreenBindGroup {
    bg: wgpu::BindGroup,
    layout: wgpu::BindGroupLayout,
    buffer: wgpu::Buffer,
}

impl ScreenBindGroup {
    pub fn new(size: &winit::dpi::PhysicalSize<u32>, device: &wgpu::Device) -> Self {
        let (buffer, bg, layout) = Self::build(device, size);
        return Self { bg, layout, buffer };
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        return &self.bg;
    }

    pub fn layout(&self) -> &wgpu::BindGroupLayout {
        return &self.layout;
    }

    fn build(
        device: &wgpu::Device,
        size: &winit::dpi::PhysicalSize<u32>,
    ) -> (wgpu::Buffer, wgpu::BindGroup, wgpu::BindGroupLayout) {
        let content = &[ScreenUniform {
            size: [size.width as u32, size.height as u32],
        }];

        let buffer_desc = wgpu::util::BufferInitDescriptor {
            label: Some("screen_buffer"),
            contents: bytemuck::cast_slice(content),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        };
        let buffer = device.create_buffer_init(&buffer_desc);

        let layout_desc = wgpu::BindGroupLayoutDescriptor {
            label: Some("screen_bind_group_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
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
            label: Some("screen_bind_group"),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        };
        let bind_group = device.create_bind_group(&bind_group_desc);

        return (buffer, bind_group, layout);
    }
}
