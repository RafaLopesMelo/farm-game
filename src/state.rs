use wgpu::util::DeviceExt;

use crate::{
    game,
    render::{
        self,
        bind_groups::{camera::CameraBindGroup, texture::TextureBindGroup},
        device::screen::Screen,
        draw::tile::{TileDrawCommand, TileDrawable},
        textures::atlas::TextureAtlas,
    },
};

pub struct State<'a> {
    game: game::Game,
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub camera_controller: render::camera::CameraController,
    camera_bind_group: CameraBindGroup,
    texture_bind_group: TextureBindGroup,
    texture_atlas: TextureAtlas,
    pub screen: Screen,
}

impl<'a> State<'a> {
    fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.screen.update(size.width, size.height);
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // The next texture to be presented to the surface
        let drawable = self.surface.get_current_texture()?;
        let image_view_desc = wgpu::TextureViewDescriptor::default();
        let image_view = drawable.texture.create_view(&image_view_desc);

        let encoder_desc = wgpu::CommandEncoderDescriptor {
            label: Some("encoder"),
        };

        // Encodes GPU operations. Can record RenderPass and ComputePass, and transfer operations
        // between driver-managed resources like Buffers and Textures
        // When finished recording, call `finish()` obtain a `CommandBuffer` that can be submitted to the queue
        let mut encoder = self.device.create_command_encoder(&encoder_desc);

        let pipeline = build_pipeline(
            &self.device,
            &self.config,
            &[
                self.camera_bind_group.layout(),
                self.texture_bind_group.layout(),
            ],
        );

        let mut draw_command = TileDrawCommand::new();

        let vertices = &TileDrawCommand::vertices(&self.screen);
        let vertex_buffer_desc = wgpu::util::BufferInitDescriptor {
            label: Some("vertex_buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        };
        let vertex_buffer = self.device.create_buffer_init(&vertex_buffer_desc);

        let color_attachment = wgpu::RenderPassColorAttachment {
            view: &image_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                store: wgpu::StoreOp::Store,
            },
        };

        let render_pass_desc = wgpu::RenderPassDescriptor {
            label: Some("render_pass"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        };

        let chunks = self.game.world().chunks_vec();
        for chunk in chunks {
            for row in chunk.tiles() {
                for tile in row {
                    let layers = self
                        .texture_atlas
                        .textures_for_tile(tile.as_ref(), self.game.world());

                    for layer in layers {
                        draw_command.add(
                            0,
                            layer
                                .iter()
                                .map(|texture| {
                                    TileDrawable::new(
                                        tile.as_ref(),
                                        self.game.camera(),
                                        &self.screen,
                                        texture.clone(),
                                    )
                                })
                                .collect::<Vec<TileDrawable>>(),
                        );
                    }
                }
            }
        }

        for layer in draw_command.layers() {
            // List of render commands in a command encoder. A render pass may contain any number of
            // drawing commands, and before/between each command the render state may be updated
            // however you wish
            let mut render_pass = encoder.begin_render_pass(&render_pass_desc);

            let instance_buffer_desc = wgpu::util::BufferInitDescriptor {
                label: Some("instance_buffer"),
                contents: bytemuck::cast_slice(&layer),
                usage: wgpu::BufferUsages::VERTEX,
            };

            let instance_buffer = self.device.create_buffer_init(&instance_buffer_desc);

            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, instance_buffer.slice(..));

            render_pass.set_pipeline(&pipeline);

            render_pass.set_bind_group(0, self.camera_bind_group.bind_group(), &[]);
            render_pass.set_bind_group(1, self.texture_bind_group.bind_group(), &[]);

            render_pass.draw(0..vertices.len() as u32, 0..layer.len() as u32);
        }

        self.queue.submit(std::iter::once(encoder.finish()));

        drawable.present();

        return Ok(());
    }

    pub fn update_camera(&mut self) {
        let intention = self.camera_controller.build_walk_intention();
        self.game.perform_walk(intention);

        self.camera_bind_group
            .write(self.game.camera(), &self.queue);
    }
}

fn build_pipeline(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    bind_groups_layouts: &[&wgpu::BindGroupLayout],
) -> wgpu::RenderPipeline {
    let shader_desc = wgpu::include_wgsl!("shaders/tile.wgsl");
    let shader = device.create_shader_module(shader_desc);

    let layout_desc = wgpu::PipelineLayoutDescriptor {
        label: Some("pipeline_layout"),
        bind_group_layouts: bind_groups_layouts,
        push_constant_ranges: &[],
    };

    let layout = device.create_pipeline_layout(&layout_desc);

    let color_state = &[Some(wgpu::ColorTargetState {
        format: config.format,
        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
        write_mask: wgpu::ColorWrites::ALL,
    })];

    let desc = wgpu::RenderPipelineDescriptor {
        label: Some("pipeline"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[render::vertex::Vertex::desc(), TileDrawCommand::desc()],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: color_state,
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        }),
        primitive: wgpu::PrimitiveState {
            // How to interpret the vertices. Here each set of 3 vertices composes a new triangle
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            // Triangles drawn in clockwise order is considered to be front-facing. So is not discarded by the culling
            front_face: wgpu::FrontFace::Ccw,
            // Determines whether certain faces of 3D objects should be rendered or discarded.
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        // Is used to avoid aliasing
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    };

    let pipeline = device.create_render_pipeline(&desc);
    return pipeline;
}
