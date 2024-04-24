#version 330 core

in vec2 uv;

uniform sampler2D texture;

out vec4 color;

void main() {
	color = vec4(texture(texture, uv).rgb, 1.0);
}
