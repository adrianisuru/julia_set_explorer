#version 140

in vec2 position;
in vec2 tex_coords;

out vec2 v_tex_coord;

void main() {
    v_tex_coord = tex_coords;
    gl_Position = vec4(position, 0.0, 1.0);
}
