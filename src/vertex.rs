#[derive(Copy, Clone)]
pub struct Vertex {
	pub position: [f32; 2],
	pub texture_position: [f32; 2],
}

glium::implement_vertex!(Vertex, position, texture_position);