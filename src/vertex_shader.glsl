#version 140

in vec2 position;
in vec2 texture_position;
out vec2 vertex_texture_position;

uniform mat4 matrix;

void main() {
	vertex_texture_position = texture_position;
	gl_Position = matrix * vec4(position, 0.0, 1.0);
}