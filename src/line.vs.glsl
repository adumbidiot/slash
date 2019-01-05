#version 330 core

layout (location = 2) in vec2 position;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * vec4(position, 0.0, 1.0);
}