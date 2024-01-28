
// Shaders for the openGL graphics pipeline


// Vertex Shader
pub const vertex_shader_source: &str = "
#version 460 core

// the vector 3 position of the vector as ints
layout (location = 0) in ivec3 aPos;

// the columns of the model matrix
layout(location = 1) in vec4 modelMatrixCol0;
layout(location = 2) in vec4 modelMatrixCol1;
layout(location = 3) in vec4 modelMatrixCol2;
layout(location = 4) in vec4 modelMatrixCol3;

layout(location = 5) in vec4 instanceColour; // the colour of this vertex (or the cube)
out vec4 colour; // Pass color to the fragment shader

// view and projection matricies
uniform mat4 view;
uniform mat4 projection;

void main() {
    // pass the colour to the frag shader
    colour = instanceColour;

    // convert pos from int to float
    vec3 pos = vec3(aPos);

    // reconstruct the model matrix
    mat4 modelMatrix = mat4(modelMatrixCol0, modelMatrixCol1, modelMatrixCol2, modelMatrixCol3);

    // calculate this vectors position
    gl_Position = projection * view * modelMatrix * vec4(pos, 1.0);
}";


// Fragment Shader
pub const fragment_shader_source: &str = "
#version 460 core

in vec4 colour; // Received from the vertex shader

out vec4 FragColor;

void main() {
    // OLD: FragColor = vec4(0.8, 0.0, 0.7, 1.0);
    FragColor = colour;
}";