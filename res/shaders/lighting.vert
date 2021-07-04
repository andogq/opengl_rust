#version 330 core
layout(location = 0) in vec4 position;

uniform mat4 u_model_matrix;
uniform mat4 u_view_matrix;
uniform mat4 u_projection_matrix;

out float light_distance;
out vec3 vs_light_direction;
out vec3 light;
out vec3 model_position;

void main() {
    vec4 ws_position = u_model_matrix * position;
    vec4 vs_position = u_view_matrix * ws_position;
    gl_Position = u_projection_matrix * vs_position;

    model_position = position.xyz;

    vec3 light_color = vec3(1.0, 1.0, 1.0);
    float light_intensity = 500000.0;
    light = light_color * light_intensity;

    vec4 ws_light = vec4(0.0, 500.0, -500.0, 0.0);

    light_distance = distance(ws_position, ws_light);
    vs_light_direction = normalize(((u_view_matrix * ws_light) - vs_position).xyz);
}