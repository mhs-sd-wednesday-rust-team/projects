use anyhow::anyhow;
use rand::Rng;
use specs::{Component, DenseVecStorage, DispatcherBuilder, Entities, Join, World, WorldExt};

use crate::board::{tile::Tile, WorldTileMap};

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl Position {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    /// Find pair of deltas, which will make us closer to `other`.
    pub fn find_direction(&self, other: &Self) -> MoveAction {
        let dx = other.x - self.x;
        let dy = other.y - self.y;

        if dx.abs() >= dy.abs() {
            MoveAction::new(dx.signum(), 0)
        } else {
            MoveAction::new(0, dy.signum())
        }
    }
}

pub fn find_free_position<'a>(
    map: &'a WorldTileMap,
    mut positions: impl Iterator<Item = &'a Position>,
) -> anyhow::Result<Position> {
    let mut rng = rand::thread_rng();

    for _ in 0..10000 {
        let x = rng.gen_range(0..map.width);
        let y = rng.gen_range(0..map.height);

        let proposed_position = Position::new(x as i64, y as i64);

        if matches!(map.board[y][x], Tile::Wall) || positions.any(|p| *p == proposed_position) {
            continue;
        }

        return Ok(proposed_position);
    }

    Err(anyhow!("failed to find a spawn position"))
}

#[derive(Component)]
pub struct Moved;

#[derive(Component, Debug, PartialEq, Eq)]
pub struct MoveAction {
    pub delta_x: i64,
    pub delta_y: i64,
}

impl MoveAction {
    pub fn new(delta_x: i64, delta_y: i64) -> Self {
        Self { delta_x, delta_y }
    }
}

struct MoveSystem;

impl<'a> specs::System<'a> for MoveSystem {
    type SystemData = (
        Entities<'a>,
        specs::WriteStorage<'a, Position>,
        specs::WriteStorage<'a, MoveAction>,
        specs::WriteStorage<'a, Moved>,
        specs::Read<'a, WorldTileMap>,
    );

    fn run(
        &mut self,
        (entities, mut positions, mut moves, mut moved, world_map): Self::SystemData,
    ) {
        moved.clear();

        for (e, pos, move_action) in (&entities, &mut positions, &moves).join() {
            let new_pos = Position {
                x: pos.x + move_action.delta_x,
                y: pos.y + move_action.delta_y,
            };

            let out_of_width = !(0 <= new_pos.x && new_pos.x < world_map.width as i64);
            let out_of_height = !(0 <= new_pos.y && new_pos.y < world_map.height as i64);

            if out_of_width || out_of_height {
                continue;
            }

            if matches!(
                world_map.board[new_pos.y as usize][new_pos.x as usize],
                Tile::Ground
            ) {
                pos.x = new_pos.x;
                pos.y = new_pos.y;
                moved.insert(e, Moved).unwrap();
            }
        }

        moves.clear();
    }
}

pub fn register(dispatcher: &mut DispatcherBuilder, world: &mut World) -> anyhow::Result<()> {
    world.register::<Position>();
    world.register::<MoveAction>();
    world.register::<Moved>();

    dispatcher.add(MoveSystem, "move_system", &[]);
    Ok(())
}

#[cfg(test)]
mod tests {
    use specs::Builder;

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
            let mut dispatcher_builder = DispatcherBuilder::new();

            world.insert(map.clone());

            register(&mut dispatcher_builder, &mut world).unwrap();

            world
                .create_entity()
                .with(Position::new(1, 1))
                .with(MoveAction::new(dx, dy))
                .build();

            let mut dispatcher = dispatcher_builder.build();
            dispatcher.dispatch(&world);

            let positions = &world.read_component::<Position>();
            let moved = &world.read_component::<Moved>();
            let entities = &world.entities();
            for (pos, e) in (positions, entities).join() {
                let actual_pos = (pos.x, pos.y);
                assert_eq!(expected_pos, actual_pos, "Move with delta ({}, {}) failed. Expected to get to ({}, {}), but got to ({}, {})",  dx, dy, expected_pos.0, expected_pos.1, actual_pos.0, actual_pos.1);
                assert!(
                    expected_pos == actual_pos || moved.contains(e),
                    "Should be moved if position changed"
                );
            }
        }
    }
}
