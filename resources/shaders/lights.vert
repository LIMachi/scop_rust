#version 330 core
layout (location = 0) in vec3 v_pos;
layout (location = 1) in vec3 v_color;

uniform mat4 projection;
uniform mat4 camera;

out vec3 color;

void main() {
    gl_Position = projection * camera * vec4(v_pos, 1.0);
    gl_PointSize = 1000. / gl_Position.z; //makes the point bigger the closer it is to the camera (only for lights)
    color = v_color;
}