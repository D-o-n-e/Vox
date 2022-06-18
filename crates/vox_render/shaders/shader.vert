#version 450

layout(location = 0) out vec3 fragColor;
layout(location = 0) in vec3 v_position;
layout(location = 1) in vec3 v_color;
layout(location = 5) in vec4 model_matrix_0;
layout(location = 6) in vec4 model_matrix_1;
layout(location = 7) in vec4 model_matrix_2;
layout(location = 8) in vec4 model_matrix_3;
layout(location = 9) in int use_image;

mat4 model_matrix = mat4(
    model_matrix_0,
    model_matrix_1,
    model_matrix_2,
    model_matrix_3
);


void main() {
    if (use_image == 0){
        gl_Position = model_matrix * vec4(v_position, 1.0);
        fragColor = v_color;
    }
}