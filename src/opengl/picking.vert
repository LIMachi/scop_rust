#version 330 core

layout (location = 0) in vec3 pos;

out vec4 v_id;

uniform int id;
uniform mat4 projection;
uniform mat4 camera;
uniform mat4 object[128];

void main() {
    gl_Position = projection * camera * object[gl_InstanceID] * vec4(pos, 1.0);
    int t = id + gl_InstanceID + 1;
    v_id = vec4(float((t & 0xFF000000) >> 24) / 255, float((t & 0xFF0000) >> 16) / 255, float((t & 0xFF00) >> 8) / 255, float(t & 0xFF) / 255);
}