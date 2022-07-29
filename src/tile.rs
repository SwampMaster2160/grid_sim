use super::texture;
use super::vertex;
use super::direction;

#[derive(Copy, Clone, PartialEq)]
pub enum Ground {
	Grass,
	Water,
	Bricks,
	Gravel,
	LeafLitter,
	Swamp,
	Sand,
}

impl Ground {
	fn texture(self) -> texture::Texture {
		match self {
			Ground::Grass => texture::Texture::Grass,
			Ground::Water => texture::Texture::Water,
			Ground::Bricks => texture::Texture::Bricks,
			Ground::Gravel => texture::Texture::Gravel,
			Ground::LeafLitter => texture::Texture::LeafLitter,
			Ground::Swamp => texture::Texture::Swamp,
			Ground::Sand => texture::Texture::Sand,
		}
	}
}

#[derive(Copy, Clone, PartialEq)]
pub enum Road {
	None,
	Gravel,
}

impl Road {
	fn texture(self) -> texture::Texture4Directional {
		match self {
			Road::None => texture::Texture4Directional::GravelRoad,
			Road::Gravel => texture::Texture4Directional::GravelRoad,
		}
	}
}

#[derive(Copy, Clone)]
pub enum Cover {
	None,
	Tree,
	TestBuilding,
	Road([Road; 4]),
}

impl Cover {
	pub fn render(&self, pos: [u16; 2]) -> Vec<vertex::Vertex> {
		match self {
			Cover::None => Vec::new(),
			Cover::Tree => texture::Texture::Tree.generate_tris(pos).to_vec(),
			Cover::TestBuilding => texture::Texture::TestBuilding.generate_tris(pos).to_vec(),
			Cover::Road (directions) => {
				let mut out = Vec::new();
				for (direction, road_quarter) in directions.iter().enumerate() {
					if !matches!(road_quarter, Road::None) {
						out.extend(road_quarter.texture().generate_tris(direction::Direction4::new(direction as u8), pos));
					}
				}
				out
			},
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
		let mut tris = self.ground.texture().generate_tris(pos).to_vec();
		tris.extend(self.cover.render(pos));
		tris
	}
}