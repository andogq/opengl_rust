#version 330 core
layout(triangles) in;
layout(triangle_strip, max_vertices = 3) out;

uniform mat4 u_model_matrix;
uniform mat4 u_view_matrix;
uniform mat4 u_projection_matrix;

in float light_distance[];
in vec3 vs_light_direction[];
in vec3 light[];
in vec3 model_position[];

out vec3 diffuse_light;
out vec3 normal;

void main() {
    vec3 side_a = model_position[1] - model_position[0];
    vec3 side_b = model_position[2] - model_position[0];

    vec3 face_normal = normalize(cross(side_a, side_b));
    vec3 vs_normal = normalize((u_view_matrix * u_model_matrix * vec4(face_normal, 0.0)).xyz);

    for (int i = 0; i < 3; i++) {
        normal = face_normal;

        float diffuse_strength = clamp(dot(vs_normal, vs_light_direction[i]), 0.0, 1.0);
        diffuse_light = (light[i] * diffuse_strength) / (light_distance[i] * light_distance[i]);

        gl_Position = gl_in[i].gl_Position;
        EmitVertex();
    }
}