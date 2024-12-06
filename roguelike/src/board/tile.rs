use specs::{Component, DenseVecStorage};

#[derive(Component, Clone, Debug)]
pub enum Tile {
    Ground,
    Wall,
    Player,
}

#[derive(Clone)]
pub enum Biome {
    Beach,
    Ocean,
    Castle,
}
