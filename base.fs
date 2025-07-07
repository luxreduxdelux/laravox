in vec2 fs_texture_point;
in vec4 fs_texture_color;

out vec4 fragColor;

uniform sampler2D texture_sample;

void main() {
    fragColor = texture(texture_sample, fs_texture_point) * fs_texture_color;
}
