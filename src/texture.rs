use super::vertex;

#[derive(Copy, Clone)]
pub enum Texture {
	Test,
	Grass,
	Water,
	Bricks,
	Select,
	BlueThing,
	RedThing,
	YellowThing,
	GreenThing
}

impl Texture {
	pub fn get_texture_id(self) -> u8 {
		match self {
			Self::Test => 0x00,
			Self::Grass => 0x01,
			Self::Water => 0x02,
			Self::Bricks => 0x03,
			Self::Select => 0x04,
			Self::BlueThing => 0x0F,
    		Self::RedThing => 0x10,
			Self::YellowThing => 0xF0,
    		Self::GreenThing => 0xFF,
		}
	}

	pub fn generate_tile_tris(self, x: i32, y: i32) -> [vertex::Vertex; 6] {
		let x_start = (x * 16) as f32;
		let x_end = ((x + 1) * 16) as f32;
		let y_start = (y * 16) as f32;
		let y_end = ((y + 1) * 16) as f32;

		let texture_id = self.get_texture_id();
		let texture_column = texture_id % 16;
		let texture_row = texture_id >> 4;
		let texture_x_start = (texture_column as f32) / 16.;
		let texture_y_start = 1. - (texture_row as f32) / 16.;
		let texture_x_end = ((texture_column + 1) as f32) / 16.;
		let texture_y_end = 1. - ((texture_row + 1) as f32) / 16.;

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
}