use specs::{Component, DenseVecStorage, Join};

use super::{position::Position, tile::Tile};

// do we need this?
#[derive(Component)]
pub struct Board {
    pub height: usize,
    pub width: usize,
}
