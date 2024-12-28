use specs::prelude::*;
use specs::Component;

use crate::combat::CombatStats;
use crate::flow::GameFlow;
use crate::flow::GameState;

use super::MobStrategy;
use super::Monster;

#[derive(Component, Clone, Copy)]
pub struct Panic {
    pub prev_strategy: MobStrategy,
}

struct PanicSystem;

impl<'a> specs::System<'a> for PanicSystem {
    type SystemData = (
        specs::Entities<'a>,
        specs::WriteStorage<'a, Monster>,
        specs::ReadStorage<'a, CombatStats>,
        specs::WriteStorage<'a, Panic>,
        specs::Read<'a, GameFlow>,
    );

    fn run(&mut self, (entities, mut monsters, stats, mut panics, game_flow): Self::SystemData) {
        let GameState::Running = game_flow.state else {
            return;
        };

        for (monster, panic) in (&mut monsters, &panics).join() {
            monster.strategy = panic.prev_strategy;
        }

        panics.clear();

        for (e, monster, stat) in (&entities, &mut monsters, &stats).join() {
            if stat.hp_ratio() < 0.4 {
                panics
                    .insert(
                        e,
                        Panic {
                            prev_strategy: monster.strategy,
                        },
                    )
                    .unwrap();
                monster.strategy = MobStrategy::Coward;
            }
        }
    }
}

pub fn register(dispatcher: &mut DispatcherBuilder, world: &mut World) -> anyhow::Result<()> {
    world.register::<Panic>();

    dispatcher.add(
        PanicSystem,
        "monster_panic_system",
        &["monster_spawn_system"],
    );

    Ok(())
}
