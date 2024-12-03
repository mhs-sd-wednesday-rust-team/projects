use specs::{Component, DenseVecStorage};

#[derive(Clone)]
pub enum TileKind {
    Ground,
    Wall,
}

#[derive(Clone)]
pub enum BiomeKind {
    Beach,
    Ocean,
    Castle,
}

#[derive(Component, Clone)]
pub struct Tile {
    pub kind: TileKind,
    pub biome: BiomeKind,
}
