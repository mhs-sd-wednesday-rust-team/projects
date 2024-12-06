use anyhow::anyhow;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use rand::seq::IteratorRandom;
use ratatui::style::{Color, Style};
use ratatui::text::{Span, Text};
use specs::prelude::*;
use specs::{Component, DenseVecStorage, DispatcherBuilder, World, WorldExt};

use crate::board::{
    position::Position,
    tile::{Tile, TileKind},
};
use crate::board::{WorldTileMap, WorldTileMapResource};
use crate::term::TermEvents;

#[derive(Component)]
pub struct Player {}

impl<'a> From<Player> for Text<'a> {
    fn from(_player: Player) -> Self {
        Span::raw("@")
            .style(Style::default().fg(Color::Yellow).bg(Color::Black))
            .into()
    }
}

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
            let new_x = (pos.x + delta_x).min(world_tile_map.width as i64).max(0);
            let new_y = (pos.y + delta_y).min(world_tile_map.height as i64).max(0);

            // eprintln!("Now at ({:?}, {:?}) = {:?}", new_x, new_y, world_tile_map.board[new_y as usize][new_x as usize].kind);
            if matches!(
                world_tile_map.board[new_y as usize][new_x as usize].kind,
                TileKind::Ground
            ) {
                pos.x = new_x;
                pos.y = new_y;
            }
        }
    }
}

impl<'a> specs::System<'a> for PlayerMoveSystem {
    type SystemData = (
        specs::WriteStorage<'a, Position>,
        specs::WriteStorage<'a, Player>,
        specs::Read<'a, TermEvents>,
        specs::Read<'a, WorldTileMapResource>,
    );

    fn run(&mut self, (mut positions, mut players, term_events, world_tile_map): Self::SystemData) {
        let world_map = &world_tile_map.0;
        for event in term_events.0.iter() {
            if let Event::Key(k) = event {
                if k.kind == KeyEventKind::Press {
                    match k.code {
                        KeyCode::Up => {
                            Self::try_move_player(world_map, &mut players, &mut positions, 0, -1)
                        }
                        KeyCode::Down => {
                            Self::try_move_player(world_map, &mut players, &mut positions, 0, 1)
                        }
                        KeyCode::Left => {
                            Self::try_move_player(world_map, &mut players, &mut positions, -1, 0)
                        }
                        KeyCode::Right => {
                            Self::try_move_player(world_map, &mut players, &mut positions, 1, 0)
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

pub fn register(dispatcher: &mut DispatcherBuilder, world: &mut World) -> anyhow::Result<()> {
    world.register::<Player>();

    let mut rng = rand::thread_rng();

    let player_spawn_position: Position;
    {
        let tiles = world.read_storage::<Tile>();
        let positions = world.read_storage::<Position>();

        let positioned_ground_tiles = (&positions, &tiles)
            .join()
            .filter(|(&ref _pos, &ref tile)| matches!(tile.kind, TileKind::Ground))
            .choose(&mut rng)
            .ok_or(anyhow!("Did not find any ground tile to spawn player"))?;

        player_spawn_position = positioned_ground_tiles.0.clone();
    }

    world
        .create_entity()
        .with(player_spawn_position.clone())
        .with(Player {})
        .build();

    dispatcher.add(PlayerMoveSystem, "player_move_system", &[]);
    Ok(())
}
