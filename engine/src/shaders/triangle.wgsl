struct Uniforms { mvp: mat4x4<f32> };
@group(0) @binding(0) var<uniform> u: Uniforms;
@group(0) @binding(1) var t: texture_2d<f32>;
@group(0) @binding(2) var s: sampler;

struct VsOut {
	@builtin(position) pos: vec4<f32>,
	@location(0) uv: vec2<f32>,
}

@vertex
fn vs(@location(0) pos: vec2<f32>, @location(1) uv: vec2<f32>) -> VsOut {
	var out: VsOut;
	out.pos = u.mvp * vec4<f32>(pos.x, pos.y, 0.0, 1.0);
	out.uv = uv;
	return out;
}

@fragment
fn fs(in: VsOut) -> @location(0) vec4<f32> {
	return textureSample(t, s, in.uv);
}
