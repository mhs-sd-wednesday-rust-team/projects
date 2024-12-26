use std::ops::DerefMut;

use rand::{thread_rng, Rng};
use specs::{
    Component, DenseVecStorage, DispatcherBuilder, Entities, Entity, Join, World, WorldExt,
};

use crate::{
    components::Position,
    flow::{GameFlow, GameState},
    monster::Monster,
    player::Player,
    turn::Turn,
};

pub mod view;

#[derive(Component)]
#[allow(dead_code)]
pub struct CombatStats {
    pub max_hp: i64,
    pub hp: i64,
    pub defense: i64,
    pub power: i64,
}

#[derive(Clone, Default, PartialEq, Eq)]
pub enum CombatFlowState {
    #[default]
    Tossing,
    Tossed {
        attacker_score: i64,
        defending_score: i64,
    },
}

#[derive(Clone)]
pub struct CombatFlow {
    attacker: Entity,
    defending: Entity,
    countdown: usize,
    state: CombatFlowState,
}

#[derive(Clone, Default)]
pub enum CombatState {
    #[default]
    NoCombat,
    Combat(CombatFlow),
}

impl CombatStats {
    #[allow(dead_code)]
    pub fn hp_ratio(&self) -> f64 {
        self.hp as f64 / self.max_hp as f64
    }
}

struct CombatSystem;

impl<'a> specs::System<'a> for CombatSystem {
    type SystemData = (
        Entities<'a>,
        specs::Write<'a, CombatState>,
        specs::Read<'a, Turn>,
        specs::WriteStorage<'a, Player>,
        specs::WriteStorage<'a, Monster>,
        specs::WriteStorage<'a, CombatStats>,
        specs::ReadStorage<'a, Position>,
        specs::Read<'a, GameFlow>,
    );

    fn run(
        &mut self,
        (entities, mut combat_state, turn, player, monsters, mut stats, positions, game_state): Self::SystemData,
    ) {
        let GameState::Running = game_state.state else {
            return;
        };

        match combat_state.deref_mut() {
            CombatState::NoCombat => {
                let (_, player_pos, player_entity) = (&player, &positions, &entities)
                    .join()
                    .next()
                    .expect("Player entity must exist");

                let mut has_collision = None;
                for (entity, _, pos) in (&entities, &monsters, &positions).join() {
                    let player_collision = pos.x == player_pos.x && pos.y == player_pos.y;
                    if player_collision {
                        has_collision = Some(entity);
                        break;
                    }
                }

                if let Some(monster_entity) = has_collision {
                    let (attacker, defending) = match *turn {
                        Turn::Game => (monster_entity, player_entity),
                        Turn::Player => (player_entity, monster_entity),
                    };

                    *combat_state = CombatState::Combat(CombatFlow {
                        attacker,
                        defending,
                        countdown: 8,
                        state: CombatFlowState::Tossing,
                    });
                }
            }
            CombatState::Combat(CombatFlow { countdown, .. }) if *countdown > 0 => {
                *countdown -= 1;
            }
            CombatState::Combat(CombatFlow {
                countdown,
                state,
                attacker,
                defending,
            }) => match state {
                CombatFlowState::Tossing => {
                    let mut rng = thread_rng();
                    let attacker_score: i64 = rng.gen_range(0..=8);
                    let defending_score: i64 = rng.gen_range(0..=8);
                    *state = CombatFlowState::Tossed {
                        attacker_score,
                        defending_score,
                    };
                    *countdown = 8;
                }
                CombatFlowState::Tossed {
                    attacker_score,
                    defending_score,
                } => {
                    let attacker_stats = stats.get(*attacker).unwrap();
                    let defending_stats = stats.get(*defending).unwrap();

                    let attack = *attacker_score + attacker_stats.power;
                    let defense = *defending_score + defending_stats.defense;

                    if attack > defense {
                        let defending_stats = stats.get_mut(*defending).unwrap();
                        defending_stats.hp -= attack - defense;
                    }

                    *combat_state = CombatState::NoCombat;
                }
            },
        };
    }
}

struct DeathSystem;

impl<'a> specs::System<'a> for DeathSystem {
    type SystemData = (
        Entities<'a>,
        specs::ReadStorage<'a, Player>,
        specs::ReadStorage<'a, Monster>,
        specs::ReadStorage<'a, CombatStats>,
        specs::Write<'a, GameFlow>,
    );

    fn run(&mut self, (entities, player, monsters, stats, mut game_flow): Self::SystemData) {
        for (player_stats, _) in (&stats, &player).join() {
            if player_stats.hp <= 0 {
                game_flow.state = GameState::Finished;
                return;
            }
        }

        for (entity, entity_stats, _) in (&entities, &stats, &monsters).join() {
            if entity_stats.hp <= 0 {
                entities
                    .delete(entity)
                    .expect("monster deletion should succeed");
            }
        }
    }
}

pub fn register(dispatcher: &mut DispatcherBuilder, world: &mut World) -> anyhow::Result<()> {
    world.insert(CombatState::default());

    world.register::<CombatStats>();

    dispatcher.add(
        CombatSystem,
        "combat_system",
        &["player_move_system", "monster_move_system"],
    );
    dispatcher.add(DeathSystem, "death_system", &["combat_system"]);
    Ok(())
}
