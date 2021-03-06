#version 330 core

uniform sampler2D tex;
uniform vec4 in_color;
in vec2 v_tex_coords;

void main() {
	//gl_FragColor = vec4(1, 0.5, 0.5, 1);// * vec4(1.0, 1.0, 1.0, texture(tex, v_tex_coords).r);
	gl_FragColor = in_color * vec4(1.0, 1.0, 1.0, texture(tex, v_tex_coords).r);
}