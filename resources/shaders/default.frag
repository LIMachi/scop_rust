#version 330 core

in vec3 pos;
in vec3 color;
in vec2 uv;
in vec3 normal;
flat in int f;

out vec4 output_color;

uniform float fade;

uniform sampler2D ambient;

uniform int light_count;
uniform vec3 lights[128];

void main() {
	int depth = 32;
	float scale = float(depth - 1);
	int face = gl_PrimitiveID / 3;
	float r = float(face * 3 % depth) / scale;
	//we repeat the process with different spacings for green and blue
	float g = float(face * 2 % depth) / scale;
	float b = float(face % depth) / scale;
	vec4 geo_color = vec4(r, r, r, 1.);

	if ((f & 1) == 1) { //light dot
		output_color = vec4(color, 1);
	} else if ((f & 2) == 2) { //debug normals
		output_color = vec4(normal * 0.5 + 0.5, 1);
	} else { //default renderer
//		vec3 accumulated_light = vec3(0.07, 0.07, 0.07); //ambient, initial luminance
		vec3 accumulated_light = vec3(0.5, 0.5, 0.5);
		for (int i = 0; i < light_count; ++i) {
			float power = 1. - clamp(distance(lights[i * 2], pos) / 500., 0., 1.); //linear light fallof over 500 units
			vec3 light_dir = normalize(lights[i * 2] - pos);
			accumulated_light += power * lights[i * 2 + 1] * max(dot(normal, light_dir), 0.);
		}
		output_color = (vec4(texture(ambient, uv).rgb, 1) * fade + /*vec4(color, 1)*/geo_color * (1 - fade)) * vec4(min(accumulated_light, 1), 1);
	}
	if ((f & 4) == 4) {
		output_color = output_color * 0.5 + vec4(0.5, 0.5, 0., 0.5);
	}
}