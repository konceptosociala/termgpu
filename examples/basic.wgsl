struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) frag_pos: vec3<f32>,
};

struct TransformUniform {
    transform_matrix: mat4x4<f32>,
    inverse_matrix: mat4x4<f32>,
};

var<push_constant> transform: TransformUniform;

@vertex
fn vs_main(
    input: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;

    out.color = input.color;
    out.frag_pos = (transform.transform_matrix * vec4<f32>(input.position, 1.0)).xyz;
    out.clip_position = vec4<f32>(out.frag_pos, 1.0);

    return out;
}

@fragment
fn fs_main(output: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(output.color, 1.0);
}