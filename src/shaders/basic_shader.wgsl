struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) instance_position: vec3<f32>, 
    @location(3) instance_size: vec3<f32>,
    @location(4) uv_offset: vec2<f32>, 
    @location(5) uv_scale: vec2<f32>
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

struct CameraUniform {
    view_project: mat4x4<f32>
};

// Camera located in group 1 (settings up in set_binding_groups) 
@group(0) @binding(0)
var<uniform> camera: CameraUniform; 

// Vertex Shader

@vertex
fn vs_main(
	input: VertexInput,
) -> VertexOutput {
	var out: VertexOutput;
	out.tex_coords = input.tex_coords * input.uv_scale + input.uv_offset;

    let world_position = input.instance_position + input.position * input.instance_size;
	
    out.clip_position = camera.view_project * vec4<f32>(world_position, 1.0);
	return out;
}

// Fragment shader

// Texture and Sampler located in group 1 (settings up in set_binding_groups)
@group(1) @binding(0)
var texture_atlas: texture_2d<f32>;
@group(1) @binding(1)
var var_sampler: sampler;

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(texture_atlas, var_sampler, input.tex_coords);
}
