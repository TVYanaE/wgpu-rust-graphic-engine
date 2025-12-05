struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) instance_position: vec2<f32>, 
    @location(3) instance_size: vec2<f32>,
    @location(4) uv_offset: vec2<f32>, 
    @location(5) uv_scale: vec2<f32>
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

// Vertex Shader

@vertex
fn vs_main(
	input: VertexInput,
) -> VertexOutput {
	var out: VertexOutput;
	out.tex_coords = input.tex_coords * input.uv_scale + input.uv_offset;

    let instance_size_vec_3 = vec3<f32>(input.instance_size, 0.0);
    let instance_position_vec_3 = vec3<f32>(input.instance_position, 0.0);

    let world_position = instance_position_vec_3 + input.position * instance_size_vec_3;
	out.clip_position = vec4<f32>(world_position, 1.0);
	return out;
}

// Fragment shader

@group(0) @binding(0)
var texture_atlas: texture_2d<f32>;
@group(0) @binding(1)
var var_sampler: sampler;

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(texture_atlas, var_sampler, input.tex_coords);
}
