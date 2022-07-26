use super::world;

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
}