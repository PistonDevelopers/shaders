#version 150 core
in vec4 color;
in vec2 pos;

out vec4 v_Color;

void main() {
    v_Color = color;
    gl_Position = vec4(pos, 0.0, 1.0);
}
