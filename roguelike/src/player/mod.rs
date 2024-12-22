use crossterm::event::{Event, KeyCode, KeyEventKind};
use specs::prelude::*;
use specs::{Component, DenseVecStorage, DispatcherBuilder, World, WorldExt};

use crate::board::tile::Tile;
use crate::board::WorldTileMap;
use crate::components::{CombatStats, Position};
use crate::flow::{GameFlow, GameState};
use crate::monster::Monster;
use crate::term::TermEvents;

pub mod view;

#[derive(Component)]
pub struct Player {}

struct PlayerMoveSystem;

impl PlayerMoveSystem {
    fn try_move_player<'a>(
        world_tile_map: &WorldTileMap,
        entities: &Entities,
        players: &WriteStorage<'a, Player>,
        monsters: &mut WriteStorage<'a, Monster>,
        positions: &mut WriteStorage<'a, Position>,
        delta_x: i64,
        delta_y: i64,
    ) -> bool {
        let new_pos = {
            let (_, pos) = (players, positions as &WriteStorage<'a, Position>)
                .join()
                .next()
                .expect("Player entity must exist");
            Position {
                x: pos.x + delta_x,
                y: pos.y + delta_y,
            }
        };

        let monsters_collision = (entities, monsters, positions as &WriteStorage<'a, Position>)
            .join()
            .find(|(_, _, pos)| pos.x == new_pos.x && pos.y == new_pos.y)
            .map(|(e, _, _)| e);

        for (_, pos) in (players, positions).join() {
            let out_of_width = !(0 <= new_pos.x && new_pos.x < world_tile_map.width as i64);
            let out_of_height = !(0 <= new_pos.y && new_pos.y < world_tile_map.height as i64);

            if out_of_width || out_of_height {
                continue;
            }

            if let Some(monster_to_delete) = monsters_collision {
                entities.delete(monster_to_delete).unwrap();

                pos.x = new_pos.x;
                pos.y = new_pos.y;
                return true;
            }

            if matches!(
                world_tile_map.board[new_pos.y as usize][new_pos.x as usize],
                Tile::Ground
            ) {
                pos.x = new_pos.x;
                pos.y = new_pos.y;
                return true;
            }
        }

        // TODO: Should be changed to false (e.g. in
        //       case player tried to move into the wall).
        true
    }
}

impl<'a> specs::System<'a> for PlayerMoveSystem {
    type SystemData = (
        Entities<'a>,
        specs::WriteStorage<'a, Position>,
        specs::WriteStorage<'a, Player>,
        specs::WriteStorage<'a, Monster>,
        specs::Read<'a, TermEvents>,
        specs::Read<'a, WorldTileMap>,
        specs::Write<'a, GameFlow>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut positions,
            players,
            mut monsters,
            term_events,
            world_tile_map,
            mut game_flow,
        ): Self::SystemData,
    ) {
        let world_map = &world_tile_map;
        for event in term_events.0.iter() {
            if let Event::Key(k) = event {
                if k.kind == KeyEventKind::Press {
                    let deltas = match k.code {
                        KeyCode::Up | KeyCode::Char('k') => Some((0, -1)),
                        KeyCode::Down | KeyCode::Char('j') => Some((0, 1)),
                        KeyCode::Left | KeyCode::Char('h') => Some((-1, 0)),
                        KeyCode::Right | KeyCode::Char('l') => Some((1, 0)),
                        _ => None,
                    };

                    if let Some((delta_x, delta_y)) = deltas {
                        let moved = Self::try_move_player(
                            world_map,
                            &entities,
                            &players,
                            &mut monsters,
                            &mut positions,
                            delta_x,
                            delta_y,
                        );
                        if moved {
                            game_flow.state =
                                GameState::Running(crate::flow::RunningState::MobsTurn)
                        }
                    }
                }
            }
        }
    }
}

pub fn register(dispatcher: &mut DispatcherBuilder, world: &mut World) -> anyhow::Result<()> {
    world.register::<CombatStats>();
    world.register::<Player>();

    dispatcher.add(PlayerMoveSystem, "player_move_system", &[]);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::board::tile::Biome;

    use super::*;

    #[test]
    fn test_player_move() {
        let map_repr = vec!["###", "#..", "#.."];

        let map_board: Vec<Vec<Tile>> = map_repr
            .into_iter()
            .map(|row_repr| {
                let mut row = vec![];
                for ch in row_repr.chars() {
                    row.push(match ch {
                        '#' => Tile::Wall,
                        '.' => Tile::Ground,
                        _ => panic!("Unknown map tile"),
                    });
                }
                row
            })
            .collect();

        let map_height = map_board.len();
        let map_width = map_board[0].len();
        let map = WorldTileMap {
            board: map_board,
            biome: Biome::Castle,
            height: map_height,
            width: map_width,
        };

        let test_cases = vec![
            ((0, 0), (1, 1)),
            ((0, 1), (1, 2)),
            ((1, 0), (2, 1)),
            ((1, 1), (2, 2)),
            ((-1, 0), (1, 1)),
            ((100, 100), (1, 1)),
        ];

        for ((dx, dy), expected_pos) in test_cases {
            let mut world = World::new();
            world.register::<Player>();
            world.register::<Position>();
            world.register::<Monster>();

            world
                .create_entity()
                .with(Position { x: 1, y: 1 })
                .with(Player {})
                .build();

            let players = &mut world.write_component::<Player>();
            let monsters = &mut world.write_component::<Monster>();
            let positions = &mut world.write_component::<Position>();
            let entities: Entities = world.entities();

            PlayerMoveSystem::try_move_player(
                &map, &entities, players, monsters, positions, dx, dy,
            );

            for (_player, pos) in (players, positions).join() {
                let actual_pos = (pos.x, pos.y);
                assert_eq!(expected_pos, actual_pos, "Move with delta ({}, {}) failed. Expected to get to ({}, {}), but got to ({}, {})",  dx, dy, expected_pos.0, expected_pos.1, actual_pos.0, actual_pos.1);
            }
        }
    }
}
