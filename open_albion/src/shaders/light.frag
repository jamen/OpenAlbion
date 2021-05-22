#version 450

layout(location = 0) in vec2 tex_coord;

layout(location = 0) out vec4 result;

layout(set = 0, binding = 0) uniform texture2D color;
layout(set = 0, binding = 1) uniform sampler samp;

void main() {
    vec4 tex = texture(sampler2D(color, samp), tex_coord);
    float mag = length(tex_coord - vec2(0.5));
    result = vec4(mix(tex.xyz, vec3(0.0), mag * mag), 1.0);
}