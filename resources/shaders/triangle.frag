#version 330 core
out vec4 color;

in vec3 pos;
in vec2 uv;
in vec3 normal;

uniform sampler2D tex;

void main() {
//	if (tex != 0) {
		color = vec4(texture(tex, uv).rgb, 1);
//	} else {
//		color = vec4(pos, 1);
//	}
}