use board::Board;
use position::Position;
use specs::{Builder, DispatcherBuilder, World, WorldExt};
use tile::Tile;

pub mod board;
pub mod position;
pub mod tile;

pub mod view;

pub fn register(_: &mut DispatcherBuilder, world: &mut World) -> anyhow::Result<()> {
    world.register::<Position>();
    world.register::<Tile>();
    world.register::<Board>();

    // TEST board
    world
        .create_entity()
        .with(Board {
            height: 60,
            width: 140,
        })
        .build();

    world
        .create_entity()
        .with(Position { x: 20, y: 2 })
        .with(Tile {
            kind: tile::TileKind::Wall,
            biome: tile::BiomeKind::Beach,
        })
        .build();

    world
        .create_entity()
        .with(Position { x: 80, y: 30 })
        .with(Tile {
            kind: tile::TileKind::Wall,
            biome: tile::BiomeKind::Ocean,
        })
        .build();

    Ok(())
}
