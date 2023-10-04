#version 140

in vec3 v_normal;
out vec4 color;

uniform vec3 u_light;

void main() {
    float brightness = dot(normalize(v_normal), normalize(u_light));
    vec3 dark_color = vec3(0.4, 0.4, 0.4);
    vec3 regular_color = vec3(0.45, 0.45, 0.45);
    color = vec4(mix(dark_color, regular_color, brightness), 1.0);
}