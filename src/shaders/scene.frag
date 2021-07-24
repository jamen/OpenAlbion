#version 450

layout(location = 0) in vec2 tex_coord;

layout(location = 0) out vec4 result;

void main() {
    result = vec4(tex_coord, 0.0, 1.0);
}