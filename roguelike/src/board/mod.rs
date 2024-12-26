use crate::components::Position;
use generator::generate_map;
use mapgen::MapBuffer;

use specs::{DispatcherBuilder, World, WorldExt};
use tile::Tile;

use self::tile::Biome;

pub mod generator;
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

    pub fn set_biome(&mut self, biome: Biome) {
        self.biome = biome
    }

    pub fn set_map(&mut self, map: &MapBuffer) {
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                let tile = if map.is_walkable(x, y) {
                    tile::Tile::Ground
                } else {
                    tile::Tile::Wall
                };

                self.board[y][x] = tile;
            }
        }
    }
}

impl Default for WorldTileMap {
    fn default() -> Self {
        Self::new_empty(BOARD_WIDTH, BOARD_HEIGHT)
    }
}

const BOARD_HEIGHT: usize = 50;
const BOARD_WIDTH: usize = 80;

pub fn register(_: &mut DispatcherBuilder, world: &mut World) -> anyhow::Result<()> {
    world.register::<Position>();

    let map = generate_map();

    let mut world_tile_map = WorldTileMap::new_empty(BOARD_WIDTH, BOARD_HEIGHT);
    world_tile_map.set_biome(tile::Biome::Castle);
    world_tile_map.set_map(&map);
    world.insert(world_tile_map);

    Ok(())
}
