use crossterm::event::{Event, KeyCode, KeyEventKind};
use specs::prelude::*;
use specs::{Component, DenseVecStorage, DispatcherBuilder, World, WorldExt};

use crate::board::tile::Tile;
use crate::board::WorldTileMap;
use crate::components::Position;
use crate::flow::{GameFlow, GameState};
use crate::term::TermEvents;

pub mod view;

#[derive(Component)]
pub struct Player {}

struct PlayerMoveSystem;

impl PlayerMoveSystem {
    fn try_move_player<'a>(
        world_tile_map: &WorldTileMap,
        players: &WriteStorage<'a, Player>,
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

        for (_, pos) in (players, positions).join() {
            let out_of_width = !(0 <= new_pos.x && new_pos.x < world_tile_map.width as i64);
            let out_of_height = !(0 <= new_pos.y && new_pos.y < world_tile_map.height as i64);

            if out_of_width || out_of_height {
                continue;
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
        specs::WriteStorage<'a, Position>,
        specs::WriteStorage<'a, Player>,
        specs::Read<'a, TermEvents>,
        specs::Read<'a, WorldTileMap>,
        specs::Read<'a, GameFlow>,
    );

    fn run(
        &mut self,
        (mut positions, players, term_events, world_tile_map, game_flow): Self::SystemData,
    ) {
        let GameState::Running = game_flow.state else {
            return;
        };

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
                        Self::try_move_player(
                            world_map,
                            &players,
                            &mut positions,
                            delta_x,
                            delta_y,
                        );
                    }
                }
            }
        }
    }
}

pub fn register(dispatcher: &mut DispatcherBuilder, world: &mut World) -> anyhow::Result<()> {
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
