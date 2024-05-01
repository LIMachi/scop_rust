#version 330 core
out vec4 output_color;

in vec3 pos;
in vec3 color;
in vec2 uv;
in vec3 normal;

uniform int flags;

uniform sampler2D tex0;

void main() {
	if ((flags & 1) != 0) {
		output_color = vec4(pos, 1);
	} else {
		output_color = vec4(texture(tex0, uv).rgb, 1) * vec4(color, 1);
	}
}