struct VertexInput {
    @location(0) vertex_pos: vec2<f32>,
    @location(1) vertex_texture_uv: vec2<f32>,
    @location(2) instance_pos: vec2<f32>,
    @location(3) texture_uv_min: vec2<f32>,
    @location(4) texture_uv_max: vec2<f32>,
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

    let texture_x = mix(in.texture_uv_min.x, in.texture_uv_max.x, in.vertex_texture_uv.x);
    let texture_y = mix(in.texture_uv_max.y, in.texture_uv_min.y, in.vertex_texture_uv.y); // Yes, it is inverted

    out.texture_coords = vec2<f32>(texture_x, texture_y);

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(t_diffuse, s_diffuse, in.texture_coords);
    return color;
}
