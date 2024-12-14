use specs::{Component, DenseVecStorage};

#[derive(Component, Clone, Copy)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

#[derive(Component)]
pub struct CombatStats {
    pub max_hp: i64,
    pub hp: i64,
    pub defense: i64,
    pub power: i64,
}