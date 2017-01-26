precision mediump float;
uniform vec4 color;

in vec2 pos;

void main() {
    gl_Position = vec4(pos, 0.0, 1.0);
}
