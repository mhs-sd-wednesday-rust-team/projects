use std::{
    ops::DerefMut,
    time::{Duration, Instant},
};

use rand::{thread_rng, Rng};
use specs::{
    Component, DenseVecStorage, DispatcherBuilder, Entities, Entity, Join, World, WorldExt,
};

use crate::{
    experience::{GainExperience, KillExperience},
    flow::{GameFlow, GameState},
    monster::Monster,
    movement::Position,
    player::Player,
    turn::Turn,
};

pub mod view;

#[derive(Component, Clone)]
pub struct Attacked;

#[derive(Component, Clone)]
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
    HpDiff {
        defending_diff: i64,
    },
}

#[derive(Clone)]
pub struct CombatFlow {
    pub attacker: Entity,
    pub defending: Entity,
    started: Instant,
    pub state: CombatFlowState,
}

#[derive(Clone, Default)]
pub enum CombatState {
    #[default]
    NoCombat,
    Combat(CombatFlow),
}

impl CombatStats {
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
        specs::WriteStorage<'a, Attacked>,
        specs::ReadStorage<'a, Position>,
        specs::Read<'a, GameFlow>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut combat_state,
            turn,
            player,
            monsters,
            mut stats,
            mut attacked,
            positions,
            game_state,
        ): Self::SystemData,
    ) {
        let GameState::Running = game_state.state else {
            return;
        };

        match combat_state.deref_mut() {
            CombatState::NoCombat => {
                for (e, _) in (&entities, &player).join() {
                    attacked.remove(e);
                }

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
                        started: Instant::now(),
                        state: CombatFlowState::Tossing,
                    });
                }
            }
            CombatState::Combat(CombatFlow {
                started,
                state,
                attacker,
                defending,
            }) if (Instant::now() - *started) > Duration::from_secs(1) => match state {
                CombatFlowState::Tossing => {
                    let mut rng = thread_rng();
                    let attacker_score: i64 = rng.gen_range(0..=8);
                    let defending_score: i64 = rng.gen_range(0..=8);
                    *state = CombatFlowState::Tossed {
                        attacker_score,
                        defending_score,
                    };
                    *started = Instant::now();
                }
                CombatFlowState::Tossed {
                    attacker_score,
                    defending_score,
                } => {
                    let attacker_stats = stats.get(*attacker).unwrap();
                    let defending_stats = stats.get(*defending).unwrap();

                    let attack = *attacker_score + attacker_stats.power;
                    let defense = *defending_score + defending_stats.defense;

                    let diff = if attack > defense {
                        defense - attack
                    } else {
                        0
                    };

                    *state = CombatFlowState::HpDiff {
                        defending_diff: diff,
                    };
                    *started = Instant::now();
                }
                CombatFlowState::HpDiff { defending_diff } => {
                    let defending_stats = stats.get_mut(*defending).unwrap();
                    attacked.insert(*attacker, Attacked).unwrap();

                    defending_stats.hp += *defending_diff;

                    *combat_state = CombatState::NoCombat;
                }
            },
            _ => {}
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
        specs::ReadStorage<'a, KillExperience>,
        specs::WriteStorage<'a, GainExperience>,
        specs::Read<'a, GameFlow>,
    );

    fn run(
        &mut self,
        (entities, player, monsters, stats, kill_experience, mut gain_experience, game_flow): Self::SystemData,
    ) {
        let GameState::Running = game_flow.state else {
            return;
        };

        let (player_entity, _) = (&entities, &player).join().next().unwrap();

        gain_experience
            .insert(player_entity, GainExperience::new(0))
            .unwrap();
        for (entity, entity_stats, _) in (&entities, &stats, &monsters).join() {
            if entity_stats.hp <= 0 {
                if let Some(kill_exp) = kill_experience.get(entity) {
                    let gain = gain_experience.get_mut(player_entity).unwrap();
                    gain.exp_count += GainExperience::from(kill_exp.clone()).exp_count;
                }

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
    world.register::<Attacked>();

    dispatcher.add(
        CombatSystem,
        "combat_system",
        &["player_move_system", "monster_move_system"],
    );
    dispatcher.add(DeathSystem, "death_system", &["combat_system"]);
    Ok(())
}
