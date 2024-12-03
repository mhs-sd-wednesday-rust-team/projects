use board::Board;
use generator::generate_map;
use position::Position;
use specs::{Builder, DispatcherBuilder, World, WorldExt};
use tile::Tile;

pub mod board;
mod generator;
pub mod position;
pub mod tile;


pub fn register(_: &mut DispatcherBuilder, world: &mut World) -> anyhow::Result<()> {
    world.register::<Position>();
    world.register::<Tile>();
    world.register::<Board>();

    // TEST board
    world
        .create_entity()
        .with(Board {
            width: 140,
            height: 60,
        })
        .build();

    let map = generate_map();

    let biome = tile::BiomeKind::Castle;

    for x in 0..140 {
        for y in 0..60 {
            let tile_kind = if map.is_walkable(x, y) {
                tile::TileKind::Ground
            } else {
                tile::TileKind::Wall
            };

            world
                .create_entity()
                .with(Position {
                    x: x as i64,
                    y: y as i64,
                })
                .with(Tile {
                    biome: biome.clone(),
                    kind: tile_kind,
                })
                .build();
        }
    }

    Ok(())
}
