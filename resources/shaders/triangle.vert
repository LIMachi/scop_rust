#version 330 core
layout (location = 0) in vec3 v_pos;
layout (location = 1) in vec3 v_color;
layout (location = 2) in vec3 v_uv;
layout (location = 3) in vec3 v_normal;

uniform mat4 object;
uniform mat4 proj;
uniform mat4 camera;
uniform uint flags;

out vec3 pos;
out vec3 color;
out vec2 uv;
out vec3 normal;

void main() {
	vec4 p = object * vec4(v_pos, 1.0);
	gl_Position = proj * camera * p;
	pos = p.xyz;
//	color = v_color;
	float t = float((gl_VertexID / 3) % 16) / 16;
	color = vec3(t, t, t);
	uv = v_uv.xy;
	normal = vec3(transpose(inverse(object)) * vec4(v_normal, 1.0));
}