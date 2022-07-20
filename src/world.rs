use super::vertex;
use super::texture;

pub struct World {

}

impl World {
	pub fn new() -> Self {
		Self {}
	}

	pub fn render(&self) -> Vec<vertex::Vertex> {
		let mut data: Vec<vertex::Vertex> = Vec::new();
		for x in 0..256u16 {
			for y in 0..256u16 {
				data.extend(texture::Texture::Grass.generate_tile_tris(x, y));
			}
		}
		data
	}
}