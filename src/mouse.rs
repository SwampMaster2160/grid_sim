use super::world;
use super::direction;

pub struct Mouse {
	pub pos: [u16; 2],
	pub click_start: [u16; 2],

	pub gui_pos: [u8; 2],

	pub is_left_clicking: bool,
	pub is_middle_clicking: bool,
	pub is_right_clicking: bool,
}

impl Mouse {
	pub fn set_pos(&mut self, pos: [i32; 2], gui_pos: [u8; 2], world: &world::World) {
		self.pos = [
			pos[0].clamp(0, world.get_width() as i32 - 1) as u16,
			pos[1].clamp(0, world.get_height() as i32 - 1) as u16,
		];
		self.gui_pos = gui_pos;
	}

	pub fn set_click_start(&mut self) {
		self.click_start = self.pos;
	}

	pub fn get_line_drag_direction(&self) -> direction::Direction2 {
		match self.pos[0].abs_diff(self.click_start[0]) > self.pos[1].abs_diff(self.click_start[1]) {
			false => direction::Direction2::NorthSouth,
			true => direction::Direction2::EastWest,
		}
	}
}