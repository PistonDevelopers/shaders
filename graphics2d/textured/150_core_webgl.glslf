precision mediump float;
uniform sampler2D s_texture;
uniform vec4 color;

in vec2 v_UV;

out vec4 o_Color;

void main()
{
    o_Color = texture(s_texture, v_UV) * color;
}
