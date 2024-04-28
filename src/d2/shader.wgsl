struct AspectUniform {
    aspect: f32
};
@group(0) @binding(0)
var<uniform> aspect: AspectUniform;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) depth: f32,
    @location(2) tex_coords: vec2<f32>,
    @location(3) tex_index: u32,
    @location(4) alpha: f32
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) tex_index: u32,
    @location(2) alpha: f32
};

@vertex
fn vs_main(
    in: VertexInput
) -> VertexOutput {
    var out: VertexOutput;

    out.tex_coords = in.tex_coords;
    out.tex_index = in.tex_index;
    out.alpha = in.alpha;

    out.clip_position = vec4(in.position, in.depth, 1.0);

    out.clip_position.x = out.clip_position.x / aspect.aspect;

    return out;
}

@group(1) @binding(0)
var t_arr: binding_array<texture_2d<f32>>;
@group(1) @binding(1)
var s: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let colour = in.alpha * textureSample(
        t_arr[in.tex_index],
        s,
        in.tex_coords
    );
    return colour;
}
