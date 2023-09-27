#version 150

in vec3 position;
in vec3 normal;

out vec3 v_normal;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;
uniform vec3 pos;

void main() {
    mat4 mvp = projection * view * model;

    v_normal = transpose(inverse(mat3(mvp))) * normal;

    gl_Position = mvp * vec4(pos + position, 1.0);
}