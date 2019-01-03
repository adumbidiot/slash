#version 330 core

precision highp float;
uniform vec4 in_color;

void main() {
    gl_FragColor = in_color;
}