#version 330 core
layout (location = 0) in vec3 v_pos;
layout (location = 1) in vec3 v_color;
layout (location = 2) in vec3 v_uv;
layout (location = 3) in vec3 v_normal;
layout (location = 4) in int v_material;

layout (location = 5) in int i_flags;
layout (location = 6) in mat4 i_mat;

out vec3 pos;
out vec3 color;
out vec2 uv;
out vec3 normal;
flat out int f;
flat out int material;

uniform mat4 projection;
uniform mat4 camera;
//uniform int flags[128];
//uniform mat4 object[128];

void main() {
	vec4 p = /*object[gl_InstanceID]*/i_mat * vec4(v_pos, 1.0);
	f = i_flags/*flags[gl_InstanceID]*/;
	material = v_material;
	gl_Position = projection * camera * p;
	pos = p.xyz;
	color = v_color;
	uv = v_uv.xy;
	normal = mat3(transpose(inverse(/*object[gl_InstanceID]*/i_mat))) * v_normal;
}