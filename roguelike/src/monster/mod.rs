use rand::distributions::{Distribution, Standard};
use rand::seq::SliceRandom;
use rand::Rng;
use specs::prelude::*;
use specs::Component;

use crate::board::tile::Tile;
use crate::board::WorldTileMap;
use crate::components::Position;
use crate::flow::{GameFlow, GameState};
use crate::player::Player;
use crate::turn::Turn;

pub mod view;

pub const DEFAULT_MONSTERS_NUMBER: usize = 10;
pub const MONSTER_SEE_DISTANCE: i64 = 10;

pub enum MobStrategy {
    Random,
    Aggressive,
    Coward,
}

impl MobStrategy {
    fn find_deltas(&self, pos: &Position, player_pos: &Position) -> (i64, i64) {
        match self {
            MobStrategy::Random => {
                let deltas = [-1, 0, 1];
                let mut rng = rand::thread_rng();
                let delta_x = *deltas.choose(&mut rng).expect("Delta must exist.");
                let delta_y = *deltas.choose(&mut rng).expect("Delta must exist.");
                (delta_x, delta_y)
            }
            MobStrategy::Aggressive => {
                let distance_to_the_player = pos.distance(player_pos);

                if distance_to_the_player < MONSTER_SEE_DISTANCE {
                    pos.find_direction(player_pos)
                } else {
                    (0, 0)
                }
            }
            MobStrategy::Coward => {
                let distance_to_the_player = pos.distance(player_pos);

                if distance_to_the_player < MONSTER_SEE_DISTANCE {
                    let (delta_x, delta_y) = pos.find_direction(player_pos);
                    (-delta_x, -delta_y)
                } else {
                    (0, 0)
                }
            }
        }
    }
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
    pub strategy: MobStrategy,
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

        for (monster, pos) in (monsters, positions).join() {
            let (delta_x, delta_y) = monster.strategy.find_deltas(pos, &player_pos);

            let new_x = pos.x + delta_x;
            let new_y = pos.y + delta_y;

            let out_of_width = !(0 <= new_x && new_x < world_tile_map.width as i64);
            let out_of_height = !(0 <= new_y && new_y < world_tile_map.height as i64);

            if out_of_width || out_of_height {
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
        specs::Read<'a, Turn>,
        specs::Read<'a, GameFlow>,
    );

    fn run(
        &mut self,
        (mut positions, players, monsters, world_tile_map, turn, game_flow): Self::SystemData,
    ) {
        let GameState::Running = game_flow.state else {
            return;
        };

        if *turn == Turn::Game {
            let world_map = &world_tile_map;
            Self::try_move_monsters(world_map, &players, &monsters, &mut positions);
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

        let proposed_position = Position::new(x as i64, y as i64);

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

    dispatcher.add(MonsterSystem, "monster_move_system", &[]);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mob_strategy_deltas_calculation() {
        let player_x = 10;
        let player_y = 10;
        let player_pos = Position::new(player_x, player_y);
        let coward_monster_pos_to_expected_delta = vec![
            (
                Position::new(player_x - MONSTER_SEE_DISTANCE, player_y),
                (0, 0),
            ),
            (
                Position::new(player_x, player_y + MONSTER_SEE_DISTANCE),
                (0, 0),
            ),
            (Position::new(player_x + 1, player_y), (1, 0)),
            (Position::new(player_x - 1, player_y), (-1, 0)),
            (Position::new(player_x, player_y + 1), (0, 1)),
            (Position::new(player_x, player_y - 1), (0, -1)),
            (Position::new(player_x - 1, player_y - 1), (-1, 0)),
            (Position::new(player_x + 1, player_y + 1), (1, 0)),
        ];

        let mut monster = Monster {
            strategy: MobStrategy::Coward,
        };
        for (monster_pos, expected_delta) in coward_monster_pos_to_expected_delta.iter() {
            let actual_delta = monster.strategy.find_deltas(monster_pos, &player_pos);
            assert_eq!(*expected_delta, actual_delta);
        }
        monster.strategy = MobStrategy::Aggressive;
        for (monster_pos, expected_delta) in coward_monster_pos_to_expected_delta.iter() {
            let expected_delta = (-expected_delta.0, -expected_delta.1);
            let actual_delta = monster.strategy.find_deltas(monster_pos, &player_pos);
            assert_eq!(expected_delta, actual_delta);
        }
    }
}
