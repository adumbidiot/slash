#version 330 core
layout (location = 0) in vec2 position;
layout (location = 1) in vec2 normal;

uniform mat4 Projection;
varying vec2 value;

void main() {
	value = normal;
	gl_Position = Projection * vec4(position, 0.0, 1.0);
}