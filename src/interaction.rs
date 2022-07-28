use crate::{texture, vertex};

use super::tile;
use super::mouse;

#[derive(Copy, Clone)]
pub enum TileInteraction {
	ReplaceGround(tile::Ground),
	BuildCover(tile::Cover),
	DemolishCover,
}

impl TileInteraction {
	pub fn interact(&self, tile: &mut tile::Tile) {
		match *self {
			TileInteraction::ReplaceGround(ground) => tile.ground = ground,
			TileInteraction::BuildCover(cover) => {
				if matches!(tile.cover, tile::Cover::None) {
					tile.cover = cover;
				}
			},
			TileInteraction::DemolishCover => tile.cover = tile::Cover::None,
		}
	}

	pub fn generate_select_tris(&self, tile: &tile::Tile, pos: [u16; 2]) -> [vertex::Vertex; 6] {
		match self {
			TileInteraction::ReplaceGround(replace_with) => {
				match *replace_with == tile.ground {
					false => texture::Texture::SelectBuildable,
					true => texture::Texture::SelectUnbuildable,
				}.generate_tris(pos)
			}
			TileInteraction::BuildCover(_) => {
				match tile.cover {
					tile::Cover::None => texture::Texture::SelectBuildable,
					_ => texture::Texture::SelectUnbuildable,
				}.generate_tris(pos)
			},
			TileInteraction::DemolishCover => {
				match tile.cover {
					tile::Cover::None => texture::Texture::SelectUnbuildable,
					_ => texture::Texture::SelectDestroy,
				}.generate_tris(pos)
			}
		}
	}
}

#[derive(Copy, Clone)]
pub enum InteractionShape {
	Dot(TileInteraction),
	Rectangle(TileInteraction),
}

impl InteractionShape {
	pub fn interact(&self, tiles: &mut ndarray::Array2<tile::Tile>, mouse: &mouse::Mouse) {
		match self {
			Self::Dot(tile_interaction) => {
				tile_interaction.interact(&mut tiles[[mouse.pos[0] as usize, mouse.pos[1] as usize]]);
			}
			Self::Rectangle(tile_interaction) => {
				for y in mouse.click_start[1].min(mouse.pos[1])..=mouse.click_start[1].max(mouse.pos[1]) {
					for x in mouse.click_start[0].min(mouse.pos[0])..=mouse.click_start[0].max(mouse.pos[0]) {
						tile_interaction.interact(&mut tiles[[x as usize, y as usize]]);
					}
				}
			}
		}
	}

	pub fn generate_select_tris(&self, tiles: &ndarray::Array2<tile::Tile>, mouse: &mouse::Mouse) -> Vec<vertex::Vertex> {
		match (self, mouse.is_left_clicking) {
			(Self::Rectangle(interaction), true) => {
				let mut tris: Vec<vertex::Vertex> = Vec::new();
				for y in mouse.click_start[1].min(mouse.pos[1])..=mouse.click_start[1].max(mouse.pos[1]) {
					for x in mouse.click_start[0].min(mouse.pos[0])..=mouse.click_start[0].max(mouse.pos[0]) {
						tris.extend(interaction.generate_select_tris(&tiles[[x as usize, y as usize]], [x, y]));
					}
				}
				tris
			}
			(Self::Dot(interaction) | Self::Rectangle(interaction), _) => {
				interaction.generate_select_tris(&tiles[[mouse.pos[0] as usize, mouse.pos[1] as usize]], mouse.pos).to_vec()
			}
			//_ => texture::Texture::Select.generate_tile_tris(build_end_x, build_end_y).to_vec()
		}
	}
}