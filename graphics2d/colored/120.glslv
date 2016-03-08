#version 120
attribute vec4 color;
attribute vec2 pos;

varying vec4 v_Color;

void main() {
    v_Color = color;
    gl_Position = vec4(pos, 0.0, 1.0);
}
