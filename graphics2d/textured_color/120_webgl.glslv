precision mediump float;
uniform sampler2D s_texture;

attribute vec2 pos;
attribute vec2 uv;
attribute vec4 color;

varying vec2 v_UV;
varying vec4 v_Color;

void main() {
    v_Color = color;
    v_UV = uv;
    gl_Position = vec4(pos, 0.0, 1.0);
}
