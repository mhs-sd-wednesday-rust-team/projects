use specs::{Component, DenseVecStorage};

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

#[allow(dead_code)]
#[derive(Component)]
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
