use mapgen::{AreaStartingPosition, BspRooms, MapBuffer, MapBuilder, NearestCorridors};

use super::{BOARD_HEIGHT, BOARD_WIDTH};

pub fn generate_map() -> MapBuffer {
    MapBuilder::new(BOARD_WIDTH, BOARD_HEIGHT)
        .with(BspRooms::new())
        .with(NearestCorridors::new())
        .with(AreaStartingPosition::new(
            mapgen::XStart::LEFT,
            mapgen::YStart::BOTTOM,
        ))
        .build()
}
