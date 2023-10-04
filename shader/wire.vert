#version 150

in vec3 position;
in vec3 normal;
in vec4 color;

out vec3 v_normal;
out vec4 v_color;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;
uniform vec3 pos;

void main() {
    mat4 mvp = projection * view * model;

    v_normal = transpose(inverse(mat3(mvp))) * normal;
    v_color = color;

    gl_Position = mvp * vec4(pos + position, 1.0);
}