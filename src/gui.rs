use crate::interaction;
use crate::mouse;
use super::vertex;
use super::texture;
use super::tile;
use counted_array::counted_array;

#[derive(Copy, Clone)]
struct Tool {
	interaction: interaction::InteractionShape,
	icon: texture::Texture,
}

counted_array!(const TOOLS: [Tool; _] = [
	Tool { icon: texture::Texture::Grass, interaction: interaction::InteractionShape::Rectangle(interaction::TileInteraction::ReplaceGround(tile::Ground::Grass)) },
	Tool { icon: texture::Texture::Water, interaction: interaction::InteractionShape::Rectangle(interaction::TileInteraction::ReplaceGround(tile::Ground::Water)) },
	Tool { icon: texture::Texture::Bricks, interaction: interaction::InteractionShape::Rectangle(interaction::TileInteraction::ReplaceGround(tile::Ground::Bricks)) },
	Tool { icon: texture::Texture::Bomb, interaction: interaction::InteractionShape::Rectangle(interaction::TileInteraction::DemolishCover) },
	Tool { icon: texture::Texture::Tree, interaction: interaction::InteractionShape::Rectangle(interaction::TileInteraction::BuildCover(tile::Cover::Tree)) },
	Tool { icon: texture::Texture::TestBuilding, interaction: interaction::InteractionShape::Dot(interaction::TileInteraction::BuildCover(tile::Cover::TestBuilding)) },
	Tool { icon: texture::Texture::Gravel, interaction: interaction::InteractionShape::Rectangle(interaction::TileInteraction::ReplaceGround(tile::Ground::Gravel)) },
	Tool { icon: texture::Texture::GravelRoadGUI, interaction: interaction::InteractionShape::RoadLine(tile::Road::Gravel) },
]);

pub struct GUI {
	pub is_open: bool
}

impl GUI {
	pub fn new() -> Self {
		Self { is_open: false }
	}

	pub fn render(&self) -> Vec<vertex::Vertex> {
		let mut tris = Vec::new();
		if self.is_open {
			for (index, tool) in TOOLS.iter().enumerate() {
				tris.extend(tool.icon.generate_tris([index as u16 % 8 + 4, index as u16 / 8 + 4]));
			}
		}
		tris
	}

	pub fn click(&mut self, mouse: &mouse::Mouse, interaction: &mut interaction::InteractionShape) {
		if mouse.gui_pos[0] > 3 && mouse.gui_pos[0] < 12 && mouse.gui_pos[1] > 3 && mouse.gui_pos[1] < 12 {
			let index = mouse.gui_pos[0] - 4 + (mouse.gui_pos[1] - 4) * 8;
			match TOOLS.get(index as usize) {
				Some(valid) => {
					*interaction = valid.interaction;
					self.is_open = false;
				},
				None => {},
			}
		}
	}
}