use generator::generate_map;
use mapgen::MapBuffer;

use specs::{DispatcherBuilder, World};
use tile::Tile;

use crate::flow::{GameFlow, GameState};

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

struct MapGenerationSystem;

impl<'a> specs::System<'a> for MapGenerationSystem {
    type SystemData = (specs::Read<'a, GameFlow>, specs::Write<'a, WorldTileMap>);

    fn run(&mut self, (game_flow, mut tile_map): Self::SystemData) {
        let GameState::Started = game_flow.state else {
            return;
        };

        let map: mapgen::MapBuffer = generate_map();
        tile_map.set_map(&map);
        tile_map.set_biome(tile::Biome::Castle);
    }
}

pub fn register(dispatcher: &mut DispatcherBuilder, world: &mut World) -> anyhow::Result<()> {
    let world_tile_map = WorldTileMap::new_empty(BOARD_WIDTH, BOARD_HEIGHT);
    world.insert(world_tile_map);

    dispatcher.add(MapGenerationSystem, "map_generation_system", &[]);

    Ok(())
}
