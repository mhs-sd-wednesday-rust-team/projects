use generator::generate_map;
use position::Position;
use specs::{DispatcherBuilder, World, WorldExt};
use tile::Tile;

use self::tile::Biome;

mod generator;
pub mod position;
pub mod tile;
pub mod view;

#[derive(Clone)]
pub struct WorldTileMap {
    pub board: Vec<Vec<Tile>>,
    pub biome: Biome,
    pub height: usize,
    pub width: usize,
}

impl WorldTileMap {
    fn new_empty(width: usize, height: usize) -> Self {
        Self {
            board: vec![vec![tile::Tile::Wall; width]; height],
            biome: tile::Biome::Castle,
            height,
            width,
        }
    }
}

impl Default for WorldTileMap {
    fn default() -> Self {
        Self::new_empty(BOARD_WIDTH, BOARD_HEIGHT)
    }
}

const BOARD_HEIGHT: usize = 60;
const BOARD_WIDTH: usize = 140;

fn generate_random_world_tile_map(world: &mut World) -> WorldTileMap {
    let map = generate_map();

    let mut world_tile_map = WorldTileMap::new_empty(BOARD_WIDTH, BOARD_HEIGHT);
    world_tile_map.biome = tile::Biome::Castle;

    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            let tile = if map.is_walkable(x, y) {
                tile::Tile::Ground
            } else {
                tile::Tile::Wall
            };

            world_tile_map.board[y][x] = tile;
        }
    }
    world_tile_map
}

pub fn register(_: &mut DispatcherBuilder, world: &mut World) -> anyhow::Result<()> {
    world.register::<Position>();
    world.register::<Tile>();
    world.register::<Board>();

    // TEST board
    world
        .create_entity()
        .with(Board {
            width: BOARD_WIDTH,
            height: BOARD_HEIGHT,
        })
        .build();

    let world_tile_map = generate_random_world_tile_map(world);
    world.insert(WorldTileMapResource(world_tile_map));

    Ok(())
}
