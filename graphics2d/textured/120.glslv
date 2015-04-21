#version 120
uniform sampler2D s_texture;
uniform vec4 color;

attribute vec2 pos;
attribute vec2 uv;

varying vec2 v_UV;

void main() {
    v_UV = uv;
    gl_Position = vec4(pos, 0.0, 1.0);
}
