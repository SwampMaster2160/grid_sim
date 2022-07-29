use crate::direction;
use crate::{texture, vertex};

use super::tile;
use super::mouse;

#[derive(Copy, Clone)]
pub enum TileInteraction {
	ReplaceGround(tile::Ground),
	BuildCover(tile::Cover),
	DemolishCover,
	BuildRoad([tile::Road; 4]),
}

impl TileInteraction {
	pub fn interact(&self, tile: &mut tile::Tile) {
		match self {
			TileInteraction::ReplaceGround(ground) => {
				if tile.cover.can_go_on_ground(*ground) {
					tile.ground = *ground
				}
			},
			TileInteraction::BuildCover(cover) => {
				if matches!(tile.cover, tile::Cover::None) && cover.can_go_on_ground(tile.ground) {
					tile.cover = *cover;
				}
			},
			TileInteraction::DemolishCover => tile.cover = tile::Cover::None,
			TileInteraction::BuildRoad(road_quarters) => {
				if !tile::Cover::Road([tile::Road::None; 4]).can_go_on_ground(tile.ground) {
					return
				}
				match &mut tile.cover {
					tile::Cover::None => tile.cover = tile::Cover::Road(*road_quarters),
					tile::Cover::Road(current_road_quarters) => {
						for (x, current_road_quarter) in current_road_quarters.iter_mut().enumerate() {
							if matches!(*current_road_quarter, tile::Road::None) {
								*current_road_quarter = road_quarters[x];
							}
						}
					}
					_ => {},
				}
			},
		}
	}

	pub fn generate_select_tris(&self, tile: &tile::Tile, pos: [u16; 2]) -> [vertex::Vertex; 6] {
		match self {
			TileInteraction::ReplaceGround(ground) => {
				match (*ground == tile.ground, tile.cover.can_go_on_ground(*ground)) {
					(false, true) => texture::Texture::SelectBuildable,
					_ => texture::Texture::SelectUnbuildable,
				}.generate_tris(pos)
			}
			TileInteraction::BuildCover(cover) => {
				match (tile.cover, cover.can_go_on_ground(tile.ground)) {
					(tile::Cover::None, true) => texture::Texture::SelectBuildable,
					_ => texture::Texture::SelectUnbuildable,
				}.generate_tris(pos)
			},
			TileInteraction::DemolishCover => {
				match tile.cover {
					tile::Cover::None => texture::Texture::SelectUnbuildable,
					_ => texture::Texture::SelectDestroy,
				}.generate_tris(pos)
			}
			TileInteraction::BuildRoad(_) => {
				match (matches!(tile.cover, tile::Cover::None) || matches!(tile.cover, tile::Cover::Road(_))) &&
				tile::Cover::Road([tile::Road::None; 4]).can_go_on_ground(tile.ground) {
					true => texture::Texture::SelectBuildable,
					false => texture::Texture::SelectUnbuildable,
				}.generate_tris(pos)
			},
		}
	}
}

#[derive(Copy, Clone)]
pub enum InteractionShape {
	Dot(TileInteraction),
	Rectangle(TileInteraction),
	RoadLine(tile::Road),
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
			Self::RoadLine(road) => {
				match mouse.get_line_drag_direction() {
					direction::Direction2::NorthSouth => {
						let x = mouse.click_start[0];
						let min = mouse.click_start[1].min(mouse.pos[1]);
						let max = mouse.click_start[1].max(mouse.pos[1]);
						if min == max {
							return
						}
						for y in min..=max {
							let mut road_quarters = [tile::Road::None; 4];
							if y != min {
								road_quarters[0] = *road;
							}
							if y != max {
								road_quarters[2] = *road;
							}
							let tile_interaction = TileInteraction::BuildRoad(road_quarters);
							tile_interaction.interact(&mut tiles[[x as usize, y as usize]]);
						}
					},
					direction::Direction2::EastWest => {
						let y = mouse.click_start[1];
						let min = mouse.click_start[0].min(mouse.pos[0]);
						let max = mouse.click_start[0].max(mouse.pos[0]);
						if min == max {
							return
						}
						for x in min..=max {
							let mut road_quarters = [tile::Road::None; 4];
							if x != min {
								road_quarters[3] = *road;
							}
							if x != max {
								road_quarters[1] = *road;
							}
							let tile_interaction = TileInteraction::BuildRoad(road_quarters);
							tile_interaction.interact(&mut tiles[[x as usize, y as usize]]);
						}
					},
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
			(Self::RoadLine(_), true) => {
				let mut tris: Vec<vertex::Vertex> = Vec::new();
				let interaction = TileInteraction::BuildRoad([tile::Road::None; 4]);
				match mouse.get_line_drag_direction() {
					direction::Direction2::NorthSouth => {
						let x = mouse.click_start[0];
						for y in mouse.click_start[1].min(mouse.pos[1])..=mouse.click_start[1].max(mouse.pos[1]) {
							tris.extend(interaction.generate_select_tris(&tiles[[x as usize, y as usize]], [x, y]));
						}
					},
					direction::Direction2::EastWest => {
						let y = mouse.click_start[1];
						for x in mouse.click_start[0].min(mouse.pos[0])..=mouse.click_start[0].max(mouse.pos[0]) {
							tris.extend(interaction.generate_select_tris(&tiles[[x as usize, y as usize]], [x, y]));
						}
					},
				}
				tris
			}
			_ => texture::Texture::Select.generate_tris(mouse.pos).to_vec()
		}
	}
}