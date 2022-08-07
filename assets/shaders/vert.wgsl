struct CameraUniform {
    view_proj: mat4x4<f32>,
    world_pos: vec3<i32>,
};
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

@group(2) @binding(0)
var<uniform> chunk_pos: vec3<i32>;

struct VertexIn {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn main(
    in: VertexIn,
) -> VertexOut {
    var out: VertexOut;
    out.tex_coords = in.tex_coords;

    var vert_pos = in.position + vec3<f32>(chunk_pos - camera.world_pos) * 32.;

    out.clip_position = camera.view_proj * vec4<f32>(vert_pos, 1.0);
    return out;
}