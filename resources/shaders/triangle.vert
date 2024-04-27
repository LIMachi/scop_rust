#version 330 core
layout (location = 0) in vec3 v_pos;
layout (location = 1) in vec3 v_uv;
layout (location = 2) in vec3 v_normal;

uniform mat4 object;
uniform mat4 proj;
uniform mat4 camera;

out vec3 pos;
out vec2 uv;
out vec3 normal;

void main() {
	normal = v_normal;
	vec4 p = object * vec4(v_pos, 1.0);
	gl_Position = proj * camera * p;
	pos = p.xyz;
	uv = v_uv.xy;
	normal = vec3(transpose(inverse(object)) * vec4(v_normal, 1.0));
}