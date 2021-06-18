#version 450

layout(location = 0) in vec4 pos;
layout(location = 1) in vec2 tex_coord;

layout(location = 0) out vec2 tex_coord_out;

void main() {
    tex_coord_out = tex_coord;
    gl_Position = pos;
}