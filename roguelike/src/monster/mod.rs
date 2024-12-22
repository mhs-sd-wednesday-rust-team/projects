use rand::distributions::{Distribution, Standard};
use rand::seq::SliceRandom;
use rand::Rng;
use specs::prelude::*;
use specs::Component;

use crate::board::tile::Tile;
use crate::board::WorldTileMap;
use crate::components::CombatStats;
use crate::components::Position;
use crate::flow::{GameFlow, GameState};
use crate::player::Player;

pub mod view;

pub const DEFAULT_MONSTERS_NUMBER: usize = 10;
pub const MONSTER_SEE_DISTANCE: i64 = 10;

enum MobStrategy {
    Random,
    Aggressive,
    Coward,
}

impl Distribution<MobStrategy> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MobStrategy {
        match rng.gen_range(0..3) {
            0 => MobStrategy::Random,
            1 => MobStrategy::Aggressive,
            _ => MobStrategy::Coward,
        }
    }
}

#[derive(Component)]
pub struct Monster {
    // Workaround for lazy entities deleting.
    // Saying that we should skip this entity handling.
    pub is_alive: bool,
    strategy: MobStrategy,
}

struct MonsterSystem;

impl MonsterSystem {
    /// Returns true in case monster killed a player.
    fn try_move_monsters<'a>(
        world_tile_map: &WorldTileMap,
        players: &WriteStorage<'a, Player>,
        monsters: &WriteStorage<'a, Monster>,
        positions: &mut WriteStorage<'a, Position>,
    ) -> bool {
        let player_pos = {
            let (_, pos) = (players, positions as &WriteStorage<'a, Position>)
                .join()
                .next()
                .expect("Player entity must exist");
            *pos
        };

        let monsters_positions: Vec<(usize, Position)> =
            (players, positions as &WriteStorage<'a, Position>)
                .join()
                .enumerate()
                .map(|(i, (_, pos))| (i, *pos))
                .collect();

        for (i, (monster, pos)) in (monsters, positions).join().enumerate() {
            if !monster.is_alive {
                continue;
            }
            let (delta_x, delta_y) = match monster.strategy {
                MobStrategy::Random => {
                    let deltas = [-1, 0, 1];
                    let mut rng = rand::thread_rng();
                    let delta_x = *deltas.choose(&mut rng).expect("Delta must exist.");
                    let delta_y = *deltas.choose(&mut rng).expect("Delta must exist.");
                    (delta_x, delta_y)
                }
                MobStrategy::Aggressive => {
                    let distance_to_the_player = pos.distance(&player_pos);

                    if distance_to_the_player < MONSTER_SEE_DISTANCE {
                        pos.find_direction(&player_pos)
                    } else {
                        (0, 0)
                    }
                }
                MobStrategy::Coward => {
                    let distance_to_the_player = pos.distance(&player_pos);

                    if distance_to_the_player < MONSTER_SEE_DISTANCE {
                        let (delta_x, delta_y) = pos.find_direction(&player_pos);
                        (-delta_x, -delta_y)
                    } else {
                        (0, 0)
                    }
                }
            };

            let new_x = pos.x + delta_x;
            let new_y = pos.y + delta_y;

            let player_collision = new_x == player_pos.x && new_y == player_pos.y;
            let monsters_collision = monsters_positions
                .iter()
                .filter(|(i_other, _)| *i_other != i)
                .any(|(_, pos)| pos.x == new_x && pos.y == new_y);

            if player_collision {
                // Killing player.
                return true;
            }

            let out_of_width = !(0 <= new_x && new_x < world_tile_map.width as i64);
            let out_of_height = !(0 <= new_y && new_y < world_tile_map.height as i64);

            if out_of_width || out_of_height || player_collision || monsters_collision {
                continue;
            }

            if matches!(
                world_tile_map.board[new_y as usize][new_x as usize],
                Tile::Ground
            ) {
                pos.x = new_x;
                pos.y = new_y;
            }
        }
        false
    }
}

impl<'a> specs::System<'a> for MonsterSystem {
    type SystemData = (
        specs::WriteStorage<'a, Position>,
        specs::WriteStorage<'a, Player>,
        specs::WriteStorage<'a, Monster>,
        specs::Read<'a, WorldTileMap>,
        specs::Write<'a, GameFlow>,
    );

    fn run(
        &mut self,
        (mut positions, players, monsters, world_tile_map, mut game_flow): Self::SystemData,
    ) {
        if game_flow.state == GameState::Running(crate::flow::RunningState::MobsTurn) {
            let world_map = &world_tile_map;
            let player_is_killed =
                Self::try_move_monsters(world_map, &players, &monsters, &mut positions);
            if player_is_killed {
                game_flow.state = GameState::Finished
            } else {
                game_flow.state = GameState::Running(crate::flow::RunningState::PlayerTurn)
            }
        }
    }
}

pub fn find_creature_spawn_position(
    map: &WorldTileMap,
    creatures_positions: &mut Vec<Position>,
) -> anyhow::Result<Position> {
    let mut rng = rand::thread_rng();

    loop {
        let x = rng.gen_range(0..map.width);
        let y = rng.gen_range(0..map.height);

        let proposed_position = Position {
            x: x as i64,
            y: y as i64,
        };

        if matches!(map.board[y][x], Tile::Wall) || creatures_positions.contains(&proposed_position)
        {
            continue;
        }

        creatures_positions.push(proposed_position);
        return Ok(proposed_position);
    }
}

pub fn register(dispatcher: &mut DispatcherBuilder, world: &mut World) -> anyhow::Result<()> {
    world.register::<Monster>();

    let mut creatures_positions = Vec::with_capacity(1 + DEFAULT_MONSTERS_NUMBER);

    let player_spawn_position = {
        let tile_map = world.read_resource::<WorldTileMap>();
        find_creature_spawn_position(&tile_map, &mut creatures_positions)?
    };

    world
        .create_entity()
        .with(player_spawn_position)
        .with(Player {})
        .with(CombatStats {
            max_hp: 30,
            hp: 30,
            defense: 2,
            power: 5,
        })
        .build();

    for _ in 0..DEFAULT_MONSTERS_NUMBER {
        let monster_spawn_position = {
            let tile_map = world.read_resource::<WorldTileMap>();
            find_creature_spawn_position(&tile_map, &mut creatures_positions)?
        };

        let strategy: MobStrategy = rand::random();

        world
            .create_entity()
            .with(monster_spawn_position)
            .with(Monster {
                strategy,
                is_alive: true,
            })
            .build();
    }

    dispatcher.add(MonsterSystem, "monster_system", &["player_move_system"]);
    Ok(())
}
