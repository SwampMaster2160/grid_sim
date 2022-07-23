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
}