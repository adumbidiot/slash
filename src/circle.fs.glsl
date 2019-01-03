#version 330 core

precision mediump float;
varying vec2 value;
uniform float border_width;

void main() {
	float radius = 1.0;
	float dist = dot(value, value);
	
	if (dist > radius){
		discard;
	}
	
	float border_radius = 1.0 - (border_width / 100.0);
	
	float sm = smoothstep(radius, radius - 0.01, dist);
	//float sm1 = smoothstep(border_radius, border_radius + 0.01, dist);
	
	float alpha = 1.0 * sm;
	
	if(dist < border_radius){
		gl_FragColor = vec4(1.0, 0.0, 1.0, alpha * 0.0);
	}else{
		gl_FragColor = vec4(0.0, 0.0, 0.0, alpha);
	}
}