use super::vertex;
use super::direction;

pub fn generate_tris_from_id(id: u8, pos: [u16; 2]) -> [vertex::Vertex; 6] {
	let x_start = (pos[0] * 16) as f32;
	let x_end = ((pos[0] + 1) * 16) as f32;
	let y_start = (pos[1] * 16) as f32;
	let y_end = ((pos[1] + 1) * 16) as f32;

	let texture_column = id % 16;
	let texture_row = id >> 4;
	let texture_x_start = (texture_column as f32) / 16.;
	let texture_y_start = 1. - ((texture_row + 1) as f32) / 16.;
	let texture_x_end = ((texture_column + 1) as f32) / 16.;
	let texture_y_end = 1. - (texture_row as f32) / 16.;

	[vertex::Vertex {
		position: [x_start, y_start],
		texture_position: [texture_x_start, texture_y_end]
	},
	vertex::Vertex {
		position: [x_end, y_start],
		texture_position: [texture_x_end, texture_y_end]
	},
	vertex::Vertex {
		position: [x_start, y_end],
		texture_position: [texture_x_start, texture_y_start]
	},
	vertex::Vertex {
		position: [x_end, y_start],
		texture_position: [texture_x_end, texture_y_end]
	},
	vertex::Vertex {
		position: [x_end, y_end],
		texture_position: [texture_x_end, texture_y_start]
	},
	vertex::Vertex {
		position: [x_start, y_end],
		texture_position: [texture_x_start, texture_y_start]
	}]
}

#[derive(Copy, Clone)]
pub enum Texture {
	//Test,
	Grass,
	Water,
	Bricks,
	Select,
	Tree,
	TestBuilding,
	SelectBuildable,
	SelectUnbuildable,
	SelectDestroy,
	Bomb,
	Gravel,
	//GravelRoad,
	GravelRoadGUI,
	//YellowThing,
	//GreenThing
}

impl Texture {
	fn get_texture_id(self) -> u8 {
		match self {
			//Self::Test => 0x00,
			Self::Grass => 0x01,
			Self::Water => 0x02,
			Self::Bricks => 0x03,
			Self::Select => 0x04,
			Self::Tree => 0x05,
			Self::TestBuilding => 0x06,
			Self::SelectBuildable => 0x07,
			Self::SelectUnbuildable => 0x08,
			Self::SelectDestroy => 0x09,
			Self::Bomb => 0x0A,
			Self::Gravel => 0x0B,
			//Self::GravelRoad => 0x0C,
			Self::GravelRoadGUI => 0x10,
			//Self::YellowThing => 0xF0,
    		//Self::GreenThing => 0xFF,
		}
	}

	pub fn generate_tris(self, pos: [u16; 2]) -> [vertex::Vertex; 6] {
		generate_tris_from_id(self.get_texture_id(), pos)
	}
}

#[derive(Copy, Clone)]
pub enum Texture4Directional {
	GravelRoad
}

impl Texture4Directional {
	fn get_texture_id(self) -> u8 {
		match self {
			Texture4Directional::GravelRoad => 0x0C,
		}
	}

	pub fn generate_tris(self, direction: direction::Direction4, pos: [u16; 2]) -> [vertex::Vertex; 6] {
		generate_tris_from_id(self.get_texture_id() + direction.get_int_val(), pos)
	}
}