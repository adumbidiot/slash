#version 330 core

layout(location = 3) in vec2 vertex;
layout(location = 4) in vec2 tex_coord;
uniform mat4 Projection;

out vec2 v_tex_coords;

void main() {
	gl_Position = vec4(vertex, 0.0, 1.0);
	v_tex_coords = tex_coord;               
}