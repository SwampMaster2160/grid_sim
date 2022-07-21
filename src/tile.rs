use super::texture;
use super::vertex;

#[derive(Copy, Clone)]
pub enum Ground {
	Grass,
	Water,
	Bricks
}

impl Ground {
	fn texture(self) -> texture::Texture {
		match self {
			Ground::Grass => texture::Texture::Grass,
			Ground::Water => texture::Texture::Water,
			Ground::Bricks => texture::Texture::Bricks,
		}
	}
}

#[derive(Clone)]
pub struct Tile {
	pub ground: Ground
}

impl Tile {
	pub fn new() -> Self {
		Self {ground: Ground::Grass}
	}

	pub fn render(&self, x: u16, y: u16) -> Vec<vertex::Vertex> {
		self.ground.texture().generate_tile_tris(x as i32, y as i32).to_vec()
	}
}