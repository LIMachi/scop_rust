#version 330 core
layout (location = 0) in vec3 pos;

uniform mat4 object;
uniform mat4 proj;
uniform mat4 camera;

void main() {
	gl_Position = proj * camera * object * vec4(pos, 1.0);
}