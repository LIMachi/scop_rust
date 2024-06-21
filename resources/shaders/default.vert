#version 330 core
layout (location = 0) in vec3 v_pos;
layout (location = 1) in vec3 v_color;
layout (location = 2) in vec3 v_uv;
layout (location = 3) in vec3 v_normal;

out vec3 pos;
out vec3 color;
out vec2 uv;
out vec3 normal;

uniform mat4 projection;
uniform mat4 camera;
uniform int flags;
uniform mat4 object[128];

void main() {
	vec4 p = object[gl_InstanceID] * vec4(v_pos, 1.0);
	gl_Position = projection * camera * p;
	if (flags == 1) {
		gl_PointSize = 1000. / gl_Position.z; //makes the point bigger the closer it is to the camera (only for lights)
	}
	pos = p.xyz;
//	color = v_color;
	//we divide the vertex id by 3 since all faces are triangles (results in an aproximation of the face id)
	//then we multiply by 3 to create a gap between faces
	//we modulus 16 to clamp the faces id from 0 to 15
	//finally we divide by 15 to get a value between 0 and 1 (full black to full red)
//	int depth = 32;
//	float scale = float(depth - 1);
//	int face = gl_VertexID / 3; //no longer works since I use indexed objects now, should probably be moved to geometry shader
//	float r = float(face * 3 % depth) / scale;
//	//we repeat the process with different spacings for green and blue
//	float g = float(face * 2 % depth) / scale;
//	float b = float(face % depth) / scale;
//	color = vec3(r, r, r);
	color = v_color;
	uv = v_uv.xy;
	normal = mat3(transpose(inverse(object[gl_InstanceID]))) * v_normal;
}