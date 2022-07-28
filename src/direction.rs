#[derive(Copy, Clone)]
pub enum Direction4 {
	North,
	East,
	South,
	West,
}

impl Direction4 {
	pub fn new(int_val: u8) -> Self {
		match int_val {
			0 => Self::North,
			1 => Self::East,
			2 => Self::South,
			3 => Self::West,
			_ => panic!(),
		}
	}

	pub fn get_int_val(self) -> u8 {
		match self {
			Self::North => 0,
			Self::East => 1,
			Self::South => 2,
			Self::West => 3,
		}
	}
}