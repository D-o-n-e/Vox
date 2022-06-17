#version 450

layout(location = 0) out vec3 fragColor;
layout(location = 0) in vec3 v_position;
layout(location = 1) in vec3 v_color;

// const vec2 positions[3] = vec2[3](
//     vec2(0.0, 0.5),
//     vec2(-0.5, -0.5),
//     vec2(0.5, -0.5)
// );

// vec3 colors[3] = vec3[](
//     vec3(1.0, 0.0, 0.0),
//     vec3(0.0, 1.0, 0.0),
//     vec3(0.0, 0.0, 1.0)
// );

void main() {
    gl_Position = vec4(v_position, 1.0);
    fragColor = v_color;
}