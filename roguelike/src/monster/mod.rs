use rand::distributions::{Distribution, Standard};
use rand::seq::SliceRandom;
use rand::Rng;
use specs::prelude::*;
use specs::Component;

use crate::board::tile::Tile;
use crate::board::WorldTileMap;
use crate::flow::{GameFlow, GameState};
use crate::movement::MoveAction;
use crate::movement::Position;
use crate::player::Player;
use crate::turn::Turn;

pub mod split_ability;
pub mod view;

pub const DEFAULT_MONSTERS_NUMBER: usize = 10;
pub const MONSTER_SEE_DISTANCE: i64 = 10;

#[derive(Clone, Copy)]
pub enum MobStrategy {
    Random,
    Aggressive,
    Coward,
}

impl MobStrategy {
    fn find_deltas(&self, pos: &Position, player_pos: &Position) -> MoveAction {
        match self {
            MobStrategy::Random => {
                let deltas = [-1, 0, 1];
                let mut rng = rand::thread_rng();
                let delta_x = *deltas.choose(&mut rng).expect("Delta must exist.");
                let delta_y = *deltas.choose(&mut rng).expect("Delta must exist.");
                MoveAction::new(delta_x, delta_y)
            }
            MobStrategy::Aggressive => {
                let distance_to_the_player = pos.distance(player_pos);

                if distance_to_the_player < MONSTER_SEE_DISTANCE {
                    pos.find_direction(player_pos)
                } else {
                    MoveAction::new(0, 0)
                }
            }
            MobStrategy::Coward => {
                let distance_to_the_player = pos.distance(player_pos);

                if distance_to_the_player < MONSTER_SEE_DISTANCE {
                    let MoveAction { delta_x, delta_y } = pos.find_direction(player_pos);
                    MoveAction::new(-delta_x, -delta_y)
                } else {
                    MoveAction::new(0, 0)
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

#[derive(Component, Clone, Copy)]
pub struct Monster {
    pub strategy: MobStrategy,
}

struct MonsterMoveSystem;

impl<'a> specs::System<'a> for MonsterMoveSystem {
    type SystemData = (
        specs::Entities<'a>,
        specs::ReadStorage<'a, Position>,
        specs::ReadStorage<'a, Player>,
        specs::ReadStorage<'a, Monster>,
        specs::WriteStorage<'a, MoveAction>,
        specs::Read<'a, Turn>,
        specs::Read<'a, GameFlow>,
    );

    fn run(
        &mut self,
        (entities, positions, players, monsters, mut moves, turn, game_flow): Self::SystemData,
    ) {
        let GameState::Running = game_flow.state else {
            return;
        };
        if *turn != Turn::Game {
            return;
        }

        let (_, player_pos) = (&players, &positions)
            .join()
            .next()
            .expect("should be a player");

        for (e, monster, pos) in (&entities, &monsters, &positions).join() {
            let move_action = monster.strategy.find_deltas(pos, player_pos);
            moves.insert(e, move_action).unwrap();
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

    dispatcher.add(MonsterMoveSystem, "monster_move_system", &[]);

    split_ability::register(dispatcher, world)?;

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
                MoveAction::new(0, 0),
            ),
            (
                Position::new(player_x, player_y + MONSTER_SEE_DISTANCE),
                MoveAction::new(0, 0),
            ),
            (Position::new(player_x + 1, player_y), MoveAction::new(1, 0)),
            (
                Position::new(player_x - 1, player_y),
                MoveAction::new(-1, 0),
            ),
            (Position::new(player_x, player_y + 1), MoveAction::new(0, 1)),
            (
                Position::new(player_x, player_y - 1),
                MoveAction::new(0, -1),
            ),
            (
                Position::new(player_x - 1, player_y - 1),
                MoveAction::new(-1, 0),
            ),
            (
                Position::new(player_x + 1, player_y + 1),
                MoveAction::new(1, 0),
            ),
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
            let expected_delta = MoveAction::new(-expected_delta.delta_x, -expected_delta.delta_y);
            let actual_delta = monster.strategy.find_deltas(monster_pos, &player_pos);
            assert_eq!(expected_delta, actual_delta);
        }
    }
}
