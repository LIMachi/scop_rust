#version 330 core
out vec4 output_color;

in vec3 color;

void main() {
    output_color = vec4(color, 1);
}