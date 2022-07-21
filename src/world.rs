use ndarray;

use super::vertex;
use super::tile;
use super::texture;

pub struct World {
	tiles: ndarray::Array2<tile::Tile>
}

impl World {
	pub fn new() -> Self {
		Self {tiles: ndarray::Array2::from_elem([256, 256], tile::Tile::new())}
	}

	pub fn build(&mut self, cursor_x: i32, cursor_y: i32) {
		let mut new_tile = tile::Tile::new();
		new_tile.ground = tile::Ground::Bricks;
		match self.tiles.get_mut([cursor_x as usize, cursor_y as usize]) {
			Some(valid_tile) => *valid_tile = new_tile,
			None => {}
		}
	}

	pub fn render(&self, cursor_x: i32, cursor_y: i32) -> Vec<vertex::Vertex> {
		let mut data: Vec<vertex::Vertex> = Vec::new();
		for x in 0..self.tiles.shape()[0] {
			for y in 0..self.tiles.shape()[1] {
				data.extend(self.tiles[[x, y]].render(x as u16, y as u16));
			}
		}
		data.extend(texture::Texture::Select.generate_tile_tris(cursor_x, cursor_y));
		data
	}
}