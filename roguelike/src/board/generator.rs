use mapgen::{
    AreaStartingPosition, BspInterior, BspRooms, CellularAutomata, MapBuffer, MapBuilder,
    MazeBuilder, NearestCorridors, NoiseGenerator, VoronoiHive,
};

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
