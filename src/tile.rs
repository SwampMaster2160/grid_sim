use super::texture;
use super::vertex;

impl Ground {

}

#[derive(Copy, Clone, PartialEq)]
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

#[derive(Copy, Clone)]
pub enum Cover {
	None,
	Tree,
	TestBuilding
}

impl Cover {
	pub fn render(&self, pos: [u16; 2]) -> Vec<vertex::Vertex> {
		match self {
			Cover::None => Vec::new(),
			Cover::Tree => texture::Texture::Tree.generate_tile_tris(pos).to_vec(),
			Cover::TestBuilding => texture::Texture::TestBuilding.generate_tile_tris(pos).to_vec(),
		}
	}
}

#[derive(Clone)]
pub struct Tile {
	pub ground: Ground,
	pub cover: Cover
}

impl Tile {
	pub fn new() -> Self {
		Self { ground: Ground::Grass, cover: Cover::None }
	}

	pub fn render(&self, pos: [u16; 2]) -> Vec<vertex::Vertex> {
		let mut tris = self.ground.texture().generate_tile_tris(pos).to_vec();
		tris.extend(self.cover.render(pos));
		tris
	}
}