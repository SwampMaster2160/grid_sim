use glium::glutin::event;
use ndarray;

use super::vertex;
use super::tile;
use super::interaction;

pub struct World {
	pub tiles: ndarray::Array2<tile::Tile>,

	pub build_start_x: u16,
	pub build_start_y: u16,
	pub build_end_x: u16,
	pub build_end_y: u16,
	is_clicking: bool,

	interaction: interaction::InteractionShape,
}

impl World {
	pub fn new() -> Self {
		Self {
			tiles: ndarray::Array2::from_elem([256, 256], tile::Tile::new()),
			build_start_x: 0, build_start_y: 0, build_end_x: 0, build_end_y: 0,
			interaction: interaction::InteractionShape::Dot(interaction::TileInteraction::ReplaceGround(tile::Ground::Bricks)),
			is_clicking: false
		}
	}

	pub fn get_width(&self) -> u16 {
		self.tiles.shape()[0] as u16
	}

	pub fn get_height(&self) -> u16 {
		self.tiles.shape()[1] as u16
	}

	pub fn set_cursor_pos(&mut self, x: i32, y: i32) {
		self.build_end_x = x.clamp(0, self.get_width() as i32 - 1) as u16;
		self.build_end_y = y.clamp(0, self.get_height() as i32 - 1) as u16;
	}

	pub fn set_build_start(&mut self) {
		self.build_start_x = self.build_end_x;
		self.build_start_y = self.build_end_y;
		self.is_clicking = true;
	}

	pub fn interact(&mut self) {
		self.interaction.interact(&mut self.tiles, self.build_start_x, self.build_start_y, self.build_end_x, self.build_end_y);
		self.is_clicking = false;
	}

	pub fn render(&self) -> Vec<vertex::Vertex> {
		let mut data: Vec<vertex::Vertex> = Vec::new();
		for (y, column) in self.tiles.axis_iter(ndarray::Axis(1)).enumerate() {
			for (x, tile) in column.iter().enumerate() {
				data.extend(tile.render(x as u16, y as u16));
			}
		}
		data.extend(self.interaction.generate_select_tris(&self.tiles, self.build_start_x, self.build_start_y, self.build_end_x, self.build_end_y, self.is_clicking));
		data
	}

	pub fn keystroke(&mut self, keycode: event::VirtualKeyCode) {
		match keycode {
			event::VirtualKeyCode::G => self.interaction = interaction::InteractionShape::Rectangle(interaction::TileInteraction::ReplaceGround(tile::Ground::Grass)),
			event::VirtualKeyCode::W => self.interaction = interaction::InteractionShape::Rectangle(interaction::TileInteraction::ReplaceGround(tile::Ground::Water)),
			event::VirtualKeyCode::B => self.interaction = interaction::InteractionShape::Rectangle(interaction::TileInteraction::ReplaceGround(tile::Ground::Bricks)),
			event::VirtualKeyCode::D => self.interaction = interaction::InteractionShape::Rectangle(interaction::TileInteraction::DemolishCover),
			event::VirtualKeyCode::T => self.interaction = interaction::InteractionShape::Rectangle(interaction::TileInteraction::BuildCover(tile::Cover::Tree)),
			event::VirtualKeyCode::H => self.interaction = interaction::InteractionShape::Dot(interaction::TileInteraction::BuildCover(tile::Cover::TestBuilding)),
			_ => {}
		}
	}
}