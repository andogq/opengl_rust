#version 330 core
layout(location = 0) in vec4 position;

uniform mat4 u_model_matrix;
uniform mat4 u_view_matrix;
uniform mat4 u_projection_matrix;

void main() {
    vec4 ws_position = u_model_matrix * position;
    vec4 vs_position = u_view_matrix * ws_position;
    gl_Position = u_projection_matrix * vs_position;
}

















