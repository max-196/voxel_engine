@group(0) @binding(0)
var atlas: texture_2d<f32>;
@group(0) @binding(1)
var atlas_sampler: sampler;

struct VertexOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@fragment
fn main(in: VertexOut) -> @location(0) vec4<f32> {
    var out = textureSample(atlas, atlas_sampler, in.tex_coords);
    return out;
}