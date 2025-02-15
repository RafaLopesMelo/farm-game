struct VertexInput {
    @location(0) vertex_pos: vec2<f32>,
    @location(1) vertex_texture_uv: vec2<f32>,
    @location(2) instance_pos: vec2<f32>,
}

struct Camera {
    coords: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> camera: Camera;

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) texture_coords: vec2<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var camera_pos: f32 = 0; // Center
    var out: VertexOutput;

    let pos = in.vertex_pos + in.instance_pos;
    let x = pos.x + camera_pos;
    let y = pos.y + camera_pos;

    out.position = vec4<f32>(x, y, 0.0, 1.0);

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    return color;
}
