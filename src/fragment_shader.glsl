#version 140

in vec2 vertex_texture_position;
out vec4 color;

uniform sampler2D texture_sampler;

void main() {
	color = texture(texture_sampler, vertex_texture_position);
}