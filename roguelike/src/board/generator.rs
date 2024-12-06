use mapgen::{AreaStartingPosition, BspRooms, MapBuffer, MapBuilder, NearestCorridors};

pub fn generate_map() -> MapBuffer {
    MapBuilder::new(140, 60)
        .with(BspRooms::new())
        .with(NearestCorridors::new())
        .with(AreaStartingPosition::new(
            mapgen::XStart::LEFT,
            mapgen::YStart::BOTTOM,
        ))
        .build()
}
