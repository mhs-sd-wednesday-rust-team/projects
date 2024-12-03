use anyhow::anyhow;
use rand::seq::IteratorRandom;
use ratatui::style::{Color, Style};
use ratatui::text::{Span, Text};
use specs::prelude::*;
use specs::{Component, DenseVecStorage, DispatcherBuilder, World, WorldExt};

use crate::board::{
    position::Position,
    tile::{Tile, TileKind},
};

#[derive(Component)]
pub struct Player {}

impl<'a> From<Player> for Text<'a>{
    fn from(_player: Player) -> Self {
        Span::raw("@")
            .style(Style::default().fg(Color::Yellow).bg(Color::Black))
            .into()
    }
}

pub fn register(_: &mut DispatcherBuilder, world: &mut World) -> anyhow::Result<()> {
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

    Ok(())
}
