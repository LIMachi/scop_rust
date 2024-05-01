#version 330 core
out vec4 color;

in vec3 pos;
in vec2 uv;
in vec3 normal;

uniform sampler2D tex2;

void main() {
	color = vec4(texture(tex2, uv).rgb, 1);
}