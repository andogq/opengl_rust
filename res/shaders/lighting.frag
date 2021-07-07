#version 330 core
layout(location = 0) out vec4 color;

in vec3 diffuse_light;
in vec3 normal;

void main() {
    // color = vec4(1.0, 0.0, 0.0, 1.0);
    // color = vec4(normalize(light_position - position) * normal * vec3(1.0, 1.0, 1.0) * 50000 / pow(distance(position, light_position), 2), 1.0);
    // color = vec4(normal, 1.0);
    color = vec4(vec3(0.8, 0.4, 0.2) * diffuse_light, 1.0);
}
