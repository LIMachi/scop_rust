#version 330 core
layout (location = 0) in vec3 pos;

uniform mat4 test;

void main() {
	gl_Position = vec4(pos.x, pos.y, pos.z, 1.0) * test;
}