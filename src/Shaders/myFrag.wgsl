


struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(1) fragColor: vec3<f32>,
};

@fragment
fn main(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(input.fragColor, 1.0);
}