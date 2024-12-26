use crossterm::event::{self, KeyCode};
use specs::prelude::*;
use specs::{DispatcherBuilder, World};

use crate::combat::CombatStats;
use crate::experience::{Experience, KillExperience};
use crate::items::{find_item_spawn_position, DEFAULT_POTIONS_NUMBER, DEFAULT_WEAPON_NUMBER};
use crate::monster::{find_creature_spawn_position, MobStrategy, Monster, DEFAULT_MONSTERS_NUMBER};
use crate::turn::Turn;
use crate::{
    board::{generator::generate_map, WorldTileMap},
    components::Position,
    player::Player,
    term::TermEvents,
};

pub mod view;

#[derive(PartialEq, Eq)]
pub enum GameState {
    Start,
    Running,
    Combat,
    Finished,
    Exit,
}

pub struct Level(usize);

impl Level {
    pub fn as_number(&self) -> usize {
        self.0 + 1
    }
}

pub struct GameFlow {
    pub state: GameState,
    pub level: Level,
}

impl GameFlow {
    pub fn new() -> Self {
        Self {
            state: GameState::Start,
            level: Level(0),
        }
    }
}

impl Default for GameFlow {
    fn default() -> Self {
        Self::new()
    }
}

struct DummyFlowSystem;

impl<'a> specs::System<'a> for DummyFlowSystem {
    type SystemData = (
        Entities<'a>,
        specs::Read<'a, TermEvents>,
        specs::Write<'a, GameFlow>,
        specs::Write<'a, Turn>,
        specs::Write<'a, WorldTileMap>,
        specs::WriteStorage<'a, Position>,
        specs::WriteStorage<'a, Player>,
        specs::WriteStorage<'a, Monster>,
        specs::WriteStorage<'a, CombatStats>,
        specs::WriteStorage<'a, Experience>,
        specs::WriteStorage<'a, KillExperience>,
    );

    fn run(
        &mut self,
        (
            entities,
            term_events,
            mut game_flow,
            mut turn,
            mut tile_map,
            mut positions,
            mut players,
            mut monsters,
            mut stats,
            mut experiences,
            mut kill_experiences,
        ): Self::SystemData,
    ) {
        for event in term_events.0.iter() {
            if let event::Event::Key(k) = event {
                if k.kind == event::KeyEventKind::Release {
                    continue;
                }

                if k.code == KeyCode::Char('q') {
                    game_flow.state = GameState::Exit;
                    return;
                }

                match game_flow.state {
                    GameState::Start => {
                        game_flow.state = GameState::Running;
                        *turn = Turn::Player;

                        let mut creatures_positions =
                            Vec::with_capacity(1 + DEFAULT_MONSTERS_NUMBER);

                        let player_spawn_position = {
                            find_creature_spawn_position(&tile_map, &mut creatures_positions)
                                .unwrap()
                        };

                        let player_entity = entities.create();
                        players.insert(player_entity, Player {}).unwrap();
                        positions
                            .insert(player_entity, player_spawn_position)
                            .unwrap();
                        stats
                            .insert(
                                player_entity,
                                CombatStats {
                                    max_hp: 30,
                                    hp: 30,
                                    defense: 2,
                                    power: 5,
                                },
                            )
                            .unwrap();
                        experiences
                            .insert(
                                player_entity,
                                Experience {
                                    level: 3,
                                    exp_count: 74,
                                },
                            )
                            .unwrap();

                        for _ in 0..DEFAULT_MONSTERS_NUMBER {
                            let monster_spawn_position = {
                                find_creature_spawn_position(&tile_map, &mut creatures_positions)
                                    .unwrap()
                            };

                            let strategy: MobStrategy = rand::random();

                            let monster_entity = entities.create();
                            monsters
                                .insert(monster_entity, Monster { strategy })
                                .unwrap();
                            positions
                                .insert(monster_entity, monster_spawn_position)
                                .unwrap();
                            stats
                                .insert(
                                    monster_entity,
                                    CombatStats {
                                        max_hp: 10,
                                        hp: 10,
                                        defense: 1,
                                        power: 5,
                                    },
                                )
                                .unwrap();
                            kill_experiences
                                .insert(monster_entity, KillExperience::new(50))
                                .unwrap();
                        }
                    }
                    GameState::Running | GameState::Combat => {
                        // FIXME: mock switch to "death".
                        if k.code == KeyCode::Char('d') {
                            game_flow.state = GameState::Finished;
                        }
                    }
                    GameState::Finished => {
                        if k.code != KeyCode::Enter {
                            continue;
                        }

                        let map: mapgen::MapBuffer = generate_map();
                        tile_map.set_map(&map);

                        // TODO: Recreate player and monsters entities instead of
                        //       changing their positions.
                        //       We have to respawn them in case player killing monsters or
                        //       a monster killing player.
                        // for (e, _) in (&entities, &players).join() {
                        //     entities.delete(e);
                        // }
                        // for (e, _) in (&entities, &monsters).join() {
                        //     entities.delete(e);
                        // }

                        let mut creatures_positions =
                            Vec::with_capacity(1 + DEFAULT_MONSTERS_NUMBER);

                        for (_, pos) in (&players, &mut positions).join() {
                            *pos =
                                find_creature_spawn_position(&tile_map, &mut creatures_positions)
                                    .unwrap_or_else(|e| panic!("{e:?}"));
                        }

                        for (_, pos) in (&monsters, &mut positions).join() {
                            *pos =
                                find_creature_spawn_position(&tile_map, &mut creatures_positions)
                                    .unwrap_or_else(|e| panic!("{e:?}"));
                        }

                        let mut items_positions =
                            Vec::with_capacity(1 + DEFAULT_POTIONS_NUMBER + DEFAULT_WEAPON_NUMBER);

                        for (_, pos) in (&monsters, &mut positions).join() {
                            *pos = find_item_spawn_position(&tile_map, &mut items_positions)
                                .unwrap_or_else(|e| panic!("{e:?}"));
                        }

                        // TODO: Should also reinitialize stats and update monsters.

                        game_flow.state = GameState::Running;
                        *turn = Turn::Player;
                    }
                    GameState::Exit => {}
                }
            }
        }
    }
}

pub fn register(dispatcher: &mut DispatcherBuilder, world: &mut World) -> anyhow::Result<()> {
    world.insert(GameFlow::default());

    dispatcher.add(DummyFlowSystem, "dummy_flow", &["death_system"]);

    Ok(())
}
