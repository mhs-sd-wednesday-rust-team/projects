use specs::{Component, DenseVecStorage};

#[derive(Component)]
#[allow(dead_code)]
pub struct CombatStats {
    pub max_hp: i64,
    pub hp: i64,
    pub defense: i64,
    pub power: i64,
}

#[derive(Component)]
pub enum CombatState {}

impl CombatStats {
    #[allow(dead_code)]
    pub fn hp_ratio(&self) -> f64 {
        self.hp as f64 / self.max_hp as f64
    }
}
