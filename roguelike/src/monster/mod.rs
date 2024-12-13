use rand::seq::SliceRandom;
use specs::prelude::*;
use specs::Component;

use crate::board::tile::Tile;
use crate::board::{position::Position, WorldTileMap};
use crate::flow::{GameFlow, GameState};
use crate::player::{find_creature_spawn_position, Player};

pub mod view;

const DEFAULT_MONSTERS_NUMBER: usize = 10;

#[derive(Component)]
pub struct Monster {}

struct MonsterSystem;

impl MonsterSystem {
    fn try_move_monsters<'a>(
        world_tile_map: &WorldTileMap,
        players: &WriteStorage<'a, Player>,
        monsters: &WriteStorage<'a, Monster>,
        positions: &mut WriteStorage<'a, Position>,
    ) {
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

        for (i, (_, pos)) in (monsters, positions).join().enumerate() {
            let deltas = [-1, 0, 1];
            let mut rng = rand::thread_rng();
            let delta_x = *deltas.choose(&mut rng).expect("Delta must exist.");
            let delta_y = *deltas.choose(&mut rng).expect("Delta must exist.");

            let new_x = pos.x + delta_x;
            let new_y = pos.y + delta_y;

            let player_collision = new_x == player_pos.x && new_y == player_pos.y;
            let monsters_collision = monsters_positions
                .iter()
                .filter(|(i_other, _)| *i_other != i)
                .any(|(_, pos)| pos.x == new_x && pos.y == new_y);

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
            Self::try_move_monsters(world_map, &players, &monsters, &mut positions);
            game_flow.state = GameState::Running(crate::flow::RunningState::PlayerTurn)
        }
    }
}

pub fn register(dispatcher: &mut DispatcherBuilder, world: &mut World) -> anyhow::Result<()> {
    world.register::<Monster>();

    for _ in 0..DEFAULT_MONSTERS_NUMBER {
        let monster_spawn_position = {
            let tile_map = world.read_resource::<WorldTileMap>();
            find_creature_spawn_position(&tile_map)?
        };

        world
            .create_entity()
            .with(monster_spawn_position)
            .with(Monster {})
            .build();
    }

    dispatcher.add(MonsterSystem, "monster_system", &["player_move_system"]);
    Ok(())
}
