#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 tex_coord;

layout(location = 0) out vec2 tex_coord_out;

layout(set = 0, binding = 0) uniform globals {
    mat4 mvp_matrix;
};

void main() {
    tex_coord_out = tex_coord;
    gl_Position = mvp_matrix * vec4(position, 1.0);
}