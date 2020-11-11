precision mediump float;
uniform sampler2D s_texture;

in vec2 v_UV;
in vec4 v_Color;

out vec4 o_Color;

void main()
{
    o_Color = texture(s_texture, v_UV) * v_Color;
}
