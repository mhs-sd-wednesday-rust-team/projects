use specs::{Component, DenseVecStorage};

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl Position {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    /// Find pair of deltas, which will make us closer to `other`.
    pub fn find_direction(&self, other: &Self) -> (i64, i64) {
        let dx = other.x - self.x;
        let dy = other.y - self.y;

        if dx.abs() >= dy.abs() {
            (dx.signum(), 0)
        } else {
            (0, dy.signum())
        }
    }
}

#[derive(Component)]
#[allow(dead_code)]
pub struct CombatStats {
    pub max_hp: i64,
    pub hp: i64,
    pub defense: i64,
    pub power: i64,
}

// #[derive(Component, Clone)]
// pub struct InBackpack {
//     pub owner: Entity,
// }

// #[derive(Component, Clone)]
// pub struct WantsToPickupItem {
//     pub collected_by: Entity,
//     pub item: Entity,
// }
