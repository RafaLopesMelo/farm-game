struct Uniforms { mvp: mat4x4<f32> };
@group(0) @binding(0) var<uniform> u: Uniforms;

@vertex
fn vs(@location(0) pos: vec2<f32>) -> @builtin(position) vec4<f32> {
	return u.mvp * vec4<f32>(pos.x, pos.y, 0.0, 1.0);
}

@fragment
fn fs() -> @location(0) vec4<f32> {
	return vec4<f32>(1.0, 0.5, 0.2, 1.0);
}
