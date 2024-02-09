

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(1) fragColor: vec4<f32>,
};

struct VertexUniforms {
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
};

// Break down the model matrix into four vec4<f32> types
struct Instance {
    @location(2) model0: vec4<f32>,
    @location(3) model1: vec4<f32>,
    @location(4) model2: vec4<f32>,
    @location(5) model3: vec4<f32>,
};


// group binding   // buffer type  // var name    // var type
@group(0) @binding(0) var<uniform> uniformBuffer: VertexUniforms;

@vertex
fn main(@location(0) position: vec3<f32>, instance: Instance, @location(6) colour: vec4<f32>) -> VertexOutput {
    var output: VertexOutput;

    // Reconstruct the model matrix from the instance data
    let model: mat4x4<f32> = mat4x4<f32>(
        instance.model0,
        instance.model1,
        instance.model2,
        instance.model3
    );

    // Multiply the position by the model matrix from the instance data
    output.pos = uniformBuffer.projection * uniformBuffer.view * model * vec4<f32>(position, 1.0);


    output.fragColor = colour;
    return output;
}