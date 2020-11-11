precision mediump float;
uniform sampler2D s_texture;

in vec2 pos;
in vec2 uv;
in vec4 color;

out vec2 v_UV;
out vec4 v_Color;

void main() {
    v_Color = color;
    v_UV = uv;
    gl_Position = vec4(pos, 0.0, 1.0);
}
