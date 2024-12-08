use specs::{Component, DenseVecStorage};

#[derive(Component, Clone, Copy)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}
