use ndarray;

use super::vertex;
use super::tile;
use super::interaction;
use super::mouse;

pub struct World {
	pub tiles: ndarray::Array2<tile::Tile>,
}

impl World {
	pub fn new() -> Self {
		Self {tiles: ndarray::Array2::from_elem([256, 256], tile::Tile::new())}
	}

	pub fn get_width(&self) -> u16 {
		self.tiles.shape()[0] as u16
	}

	pub fn get_height(&self) -> u16 {
		self.tiles.shape()[1] as u16
	}

	pub fn interact(&mut self, interaction: &interaction::InteractionShape, mouse: &mouse::Mouse) {
		interaction.interact(&mut self.tiles, mouse);
	}

	pub fn render(&self, interaction: &interaction::InteractionShape, mouse: &mouse::Mouse, is_paused: bool) -> Vec<vertex::Vertex> {
		let mut data: Vec<vertex::Vertex> = Vec::new();
		for (y, column) in self.tiles.axis_iter(ndarray::Axis(1)).enumerate() {
			for (x, tile) in column.iter().enumerate() {
				data.extend(tile.render([x as u16, y as u16]));
			}
		}
		if !is_paused {
			data.extend(interaction.generate_select_tris(&self.tiles, mouse));
		}
		data
	}
}