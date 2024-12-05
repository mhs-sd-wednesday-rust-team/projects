use board::Board;
use generator::generate_map;
use position::Position;
use specs::{Builder, DispatcherBuilder, World, WorldExt};
use tile::Tile;

pub mod board;
mod generator;
pub mod position;
pub mod tile;

#[derive(Default)]
pub struct WorldTileMap{
    pub board: Vec<Vec<Tile>>,
    pub height: usize,
    pub width: usize,
}

#[derive(Default)]
pub struct WorldTileMapResource(pub WorldTileMap);

impl WorldTileMap {

    fn new_empty(width: usize, height: usize) -> Self {
        Self { board: vec![vec![Tile { kind: tile::TileKind::Wall, biome: tile::BiomeKind::Castle }; width]; height], height: height, width: width }
    }

    // FIXME: potentially buggy indexing. Check
    pub fn xy_idx(x: i64, y: i64) -> usize {
        (y as usize * BOARD_HEIGHT) + x as usize
    }
}

const BOARD_HEIGHT: usize = 60;
const BOARD_WIDTH: usize = 140;

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

    let map = generate_map();

    let biome = tile::BiomeKind::Castle;

    let mut world_tile_map = WorldTileMap::new_empty(BOARD_WIDTH, BOARD_HEIGHT);

    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            let tile_kind = if map.is_walkable(x, y) {
                tile::TileKind::Ground
            } else {
                tile::TileKind::Wall
            };

            let tile = Tile {
                biome: biome.clone(),
                kind: tile_kind,
            };

            world
                .create_entity()
                .with(Position {
                    x: x as i64,
                    y: y as i64,
                })
                .with(tile.clone())
                .build();

            world_tile_map.board[y][x] = tile;
        }
    }

    world.insert(WorldTileMapResource(world_tile_map));


    Ok(())
}
