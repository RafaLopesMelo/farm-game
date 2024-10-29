struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) instance_position: vec2<f32>,
}

struct Screen {
    size: vec2<u32>, // [width, height]
}

@group(0) @binding(0)
var<uniform> screen: Screen;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
}

fn to_ndc(pixels: f32, physical_size: u32) -> f32 {
    return pixels / f32(physical_size) * 2.0 - 1.0;
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let pos = input.position + input.instance_position;
    let x = to_ndc(f32(pos.x), u32(screen.size.x));
    let y = to_ndc(f32(pos.y), u32(screen.size.y));
    out.position = vec4<f32>(x, y, 0.0, 1.0);

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let grass_color = vec4<f32>(0.0, 0.8, 0.0, 1.0);
    return grass_color;
}
