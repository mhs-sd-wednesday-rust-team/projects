use anyhow::anyhow;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use rand::seq::IteratorRandom;
use specs::prelude::*;
use specs::{Component, DenseVecStorage, DispatcherBuilder, World, WorldExt};

use crate::board::WorldTileMap;
use crate::board::{position::Position, tile::Tile};
use crate::term::TermEvents;

pub mod view;

#[derive(Component)]
pub struct Player {}

struct PlayerMoveSystem;

impl PlayerMoveSystem {
    fn try_move_player<'a>(
        world_tile_map: &WorldTileMap,
        players: &mut specs::WriteStorage<'a, Player>,
        positions: &mut specs::WriteStorage<'a, Position>,
        delta_x: i64,
        delta_y: i64,
    ) {
        for (_player, pos) in (players, positions).join() {
            let new_x = pos.x + delta_x;
            let new_y = pos.y + delta_y;

            if !(0 <= new_x && new_x < world_tile_map.width as i64) {
                continue;
            }

            if !(0 <= new_y && new_y < world_tile_map.height as i64) {
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
    }
}

impl<'a> specs::System<'a> for PlayerMoveSystem {
    type SystemData = (
        Entities<'a>,
        specs::WriteStorage<'a, Position>,
        specs::WriteStorage<'a, Player>,
        specs::Read<'a, TermEvents>,
        specs::Read<'a, WorldTileMap>,
    );

    fn run(
        &mut self,
        (entities, mut positions, mut players, term_events, world_tile_map): Self::SystemData,
    ) {
        let world_map = &world_tile_map;
        for event in term_events.0.iter() {
            if let Event::Key(k) = event {
                if k.kind == KeyEventKind::Press {
                    match k.code {
                        KeyCode::Char('y') => {
                            for (_, e) in (&players, &entities).join() {
                                entities.delete(e).unwrap();
                            }
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            Self::try_move_player(world_map, &mut players, &mut positions, 0, -1);
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            Self::try_move_player(world_map, &mut players, &mut positions, 0, 1);
                        }
                        KeyCode::Left | KeyCode::Char('h') => {
                            Self::try_move_player(world_map, &mut players, &mut positions, -1, 0);
                        }
                        KeyCode::Right | KeyCode::Char('l') => {
                            Self::try_move_player(world_map, &mut players, &mut positions, 1, 0);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

pub fn find_player_spawn_position(map: &WorldTileMap) -> anyhow::Result<Position> {
    let mut rng = rand::thread_rng();

    let spawn_position = (0..map.height)
        .zip(0..map.width)
        .filter(|&pos| matches!(map.board[pos.0][pos.1], Tile::Ground))
        .choose(&mut rng)
        .ok_or(anyhow!("Did not find any ground tile to spawn player"))?;

    let pos = Position {
        x: spawn_position.1 as i64,
        y: spawn_position.0 as i64,
    };
    Ok(pos)
}

pub fn register(dispatcher: &mut DispatcherBuilder, world: &mut World) -> anyhow::Result<()> {
    world.register::<Player>();

    let player_spawn_position = {
        let tile_map = world.read_resource::<WorldTileMap>();
        find_player_spawn_position(&tile_map)?
    };

    world
        .create_entity()
        .with(player_spawn_position)
        .with(Player {})
        .build();

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

            world
                .create_entity()
                .with(Position { x: 1, y: 1 })
                .with(Player {})
                .build();

            let players = &mut world.write_component::<Player>();
            let positions = &mut world.write_component::<Position>();

            PlayerMoveSystem::try_move_player(&map, players, positions, dx, dy);

            for (_player, pos) in (players, positions).join() {
                let actual_pos = (pos.x, pos.y);
                assert_eq!(expected_pos, actual_pos, "Move with delta ({}, {}) failed. Expected to get to ({}, {}), but got to ({}, {})",  dx, dy, expected_pos.0, expected_pos.1, actual_pos.0, actual_pos.1);
            }
        }
    }
}
