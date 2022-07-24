use crate::{world, texture, vertex};

use super::tile;

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

	pub fn generate_select_tris(&self, tile: &tile::Tile, x: u16, y: u16) -> [vertex::Vertex; 6] {
		match self {
			TileInteraction::ReplaceGround(replace_with) => {
				match *replace_with == tile.ground {
					false => texture::Texture::SelectBuildable,
					true => texture::Texture::SelectUnbuildable,
				}.generate_tile_tris(x, y)
			}
			TileInteraction::BuildCover(_) => {
				match tile.cover {
					tile::Cover::None => texture::Texture::SelectBuildable,
					_ => texture::Texture::SelectUnbuildable,
				}.generate_tile_tris(x, y)
			},
			TileInteraction::DemolishCover => {
				match tile.cover {
					tile::Cover::None => texture::Texture::SelectUnbuildable,
					_ => texture::Texture::SelectDestroy,
				}.generate_tile_tris(x, y)
			}
		}
	}
}

pub enum InteractionShape {
	Dot(TileInteraction),
	Rectangle(TileInteraction),
}

impl InteractionShape {
	pub fn interact(&self, tiles: &mut ndarray::Array2<tile::Tile>, build_start_x: u16, build_start_y: u16, build_end_x: u16, build_end_y: u16) {
		match self {
			Self::Dot(tile_interaction) => {
				tile_interaction.interact(&mut tiles[[build_end_x as usize, build_end_y as usize]]);
			}
			Self::Rectangle(tile_interaction) => {
				for y in build_start_y.min(build_end_y)..=build_start_y.max(build_end_y) {
					for x in build_start_x.min(build_end_x)..=build_start_x.max(build_end_x) {
						tile_interaction.interact(&mut tiles[[x as usize, y as usize]]);
					}
				}
			}
		}
	}

	pub fn generate_select_tris(&self, tiles: &ndarray::Array2<tile::Tile>, build_start_x: u16, build_start_y: u16, build_end_x: u16, build_end_y: u16, is_clicking: bool) -> Vec<vertex::Vertex> {
		match (self, is_clicking) {
			(Self::Rectangle(interaction), true) => {
				let mut tris: Vec<vertex::Vertex> = Vec::new();
				for y in build_start_y.min(build_end_y)..=build_start_y.max(build_end_y) {
					for x in build_start_x.min(build_end_x)..=build_start_x.max(build_end_x) {
						tris.extend(interaction.generate_select_tris(&tiles[[x as usize, y as usize]], x, y));
					}
				}
				tris
			}
			(Self::Dot(interaction) | Self::Rectangle(interaction), _) => {
				interaction.generate_select_tris(&tiles[[build_end_x as usize, build_end_y as usize]], build_end_x, build_end_y).to_vec()
			}
			_ => texture::Texture::Select.generate_tile_tris(build_end_x, build_end_y).to_vec()
		}
	}
}