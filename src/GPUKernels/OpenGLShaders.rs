
// Shaders for the openGL graphics pipeline


// Vertex Shader
pub const vertex_shader_source: &str = "
#version 460 core
layout (location = 0) in ivec3 aPos;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    vec3 pos = vec3(aPos);
    gl_Position = projection * view * model * vec4(pos, 1.0);
}";


// Fragment Shader
pub const fragment_shader_source: &str = "
#version 460 core
out vec4 FragColor;

void main() {
    FragColor = vec4(0.8, 0.0, 0.7, 1.0);
}";