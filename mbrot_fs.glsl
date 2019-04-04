#version 140

in vec2 v_tex_coord;

out vec4 color;

uniform sampler1D tex;
uniform int bailout;
uniform vec2 center;
uniform float radius;

void main() {
    vec2 z;
    vec2 c;
//    z.x = 3.0 * (v_tex_coord.x - 0.5);
//    z.y = 2.0 * (v_tex_coord.y - 0.5);
      c.x = radius*2 * (v_tex_coord.x - 0.5) + center.x;
      c.y = radius*2 * (v_tex_coord.y - 0.5) + center.y;
      
    int i;
    z = c;
    for(i=0; i<bailout; i++) {
        float x = (z.x * z.x - z.y * z.y) + c.x;
        float y = (z.y * z.x + z.x * z.y) + c.y;

        if((x * x + y * y) > 4.0) break;
        z.x = x;
        z.y = y;
    }

    color = texture1D(tex, (i == bailout ? 0.0 : float(i)) / 100.0);
    
}
