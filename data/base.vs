in vec2 vs_vertex_point;
in vec2 vs_texture_point;
in vec4 vs_texture_color;

out vec2 fs_texture_point;
out vec4 fs_texture_color;

uniform mat4 view_projection;

void main() {
    gl_Position = view_projection * vec4(vs_vertex_point, 0.0, 1.0);

    fs_texture_point = vs_texture_point;
    fs_texture_color = vs_texture_color;
}
