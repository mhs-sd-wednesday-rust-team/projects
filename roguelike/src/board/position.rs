use specs::{Component, DenseVecStorage};

#[derive(Component)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}
