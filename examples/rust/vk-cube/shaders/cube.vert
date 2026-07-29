#version 450

// Per-vertex cube geometry; transformed by a push-constant MVP.
layout(push_constant) uniform Push {
    mat4 mvp;
} pc;

layout(location = 0) in vec3 a_pos;
layout(location = 1) in vec3 a_color;

layout(location = 0) out vec3 v_color;

void main() {
    gl_Position = pc.mvp * vec4(a_pos, 1.0);
    v_color = a_color;
}
