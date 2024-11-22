struct VertexInput {
    @location(0) vertex_pos: vec2<u32>,
    @location(1) vertex_texture_uv: vec2<f32>,
    @location(2) instance_coords: vec3<i32>,
    @location(3) instance_offset: vec2<i32>,
    @location(4) kind: u32,
    @location(5) texture_uv_min: vec2<f32>,
    @location(6) texture_uv_max: vec2<f32>,
}

struct Screen {
    size: vec2<u32>, // [width, height]
}

@group(0) @binding(0)
var<uniform> screen: Screen;

struct Camera {
    coords: vec2<i32>,
}

@group(1) @binding(0)
var<uniform> camera: Camera;

@group(2) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(2) @binding(1)
var s_diffuse: sampler;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) kind: u32,
    @location(2) instance_coords: vec3<i32>,
    @location(3) texture_coords: vec2<f32>,
}

fn to_ndc(pixels: i32, physical_size: u32) -> f32 {
    return f32(pixels) / f32(physical_size);
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var camera_pos: f32 = 0; // Center
    var out: VertexOutput;

    let offset = vec2<i32>(in.vertex_pos) + in.instance_offset;
    let offset_x = to_ndc(offset.x, screen.size.x);
    let offset_y = to_ndc(offset.y, screen.size.y);

    let x = camera_pos + offset_x;
    let y = camera_pos + offset_y;
    out.position = vec4<f32>(x, y, 0.0, 1.0);

    out.instance_coords = in.instance_coords;
    out.kind = in.kind;

    let texture_x = mix(in.texture_uv_min.x, in.texture_uv_max.x, in.vertex_texture_uv.x);
    let texture_y = mix(in.texture_uv_min.y, in.texture_uv_max.y, in.vertex_texture_uv.y);
    out.texture_coords = vec2<f32>(texture_x, texture_y);

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var MAX_HEIGHT = 255.0;

    let is_camera_x = camera.coords.x == in.instance_coords.x;
    let is_camera_y = camera.coords.y == in.instance_coords.y;
    let is_camera = is_camera_x && is_camera_y;

    if is_camera {
        let USER_COLOR = vec4<f32>(0.74, 0.21, 0.33, 1.0);
        return USER_COLOR;
    } else {
        let color = textureSample(t_diffuse, s_diffuse, in.texture_coords);
        let brightness_factor = 1.0 - f32(in.instance_coords.z) / MAX_HEIGHT * 20;
        return color * brightness_factor;
    }
}
