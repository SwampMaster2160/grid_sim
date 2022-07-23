use ndarray;

use super::vertex;
use super::tile;
use super::texture;

pub struct World {
	tiles: ndarray::Array2<tile::Tile>,

	build_start_x: u16,
	build_start_y: u16,
	build_end_x: u16,
	build_end_y: u16,
}

impl World {
	pub fn new() -> Self {
		Self {
			tiles: ndarray::Array2::from_elem([256, 256], tile::Tile::new()),
			build_start_x: 0, build_start_y: 0, build_end_x: 0, build_end_y: 0
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
	}

	pub fn build(&mut self) {
		let mut new_tile = tile::Tile::new();
		new_tile.ground = tile::Ground::Bricks;
		new_tile.cover = tile::Cover::Tree;
		*self.tiles.get_mut([self.build_end_x as usize, self.build_end_y as usize]).unwrap() = new_tile;
	}

	pub fn render(&self) -> Vec<vertex::Vertex> {
		let mut data: Vec<vertex::Vertex> = Vec::new();
		for (y, column) in self.tiles.axis_iter(ndarray::Axis(1)).enumerate() {
			for (x, tile) in column.iter().enumerate() {
				data.extend(tile.render(x as u16, y as u16));
			}
		}
		data.extend(texture::Texture::Select.generate_tile_tris(self.build_end_x, self.build_end_y));
		data
	}
}