use flow::{GameFlow, GameState};
use specs::{DispatcherBuilder, World, WorldExt};
use std::io::Result;

mod board;
mod flow;
mod player;
mod render;
mod term;

fn main() -> Result<()> {
    let mut world = World::new();
    let mut dispatcher_builder = DispatcherBuilder::new();

    term::register(&mut dispatcher_builder, &mut world).unwrap();
    board::register(&mut dispatcher_builder, &mut world).unwrap();
    flow::register(&mut dispatcher_builder, &mut world).unwrap();
    player::register(&mut dispatcher_builder, &mut world).unwrap();
    render::register(&mut dispatcher_builder, &mut world).unwrap();

    let mut dispatcher = dispatcher_builder.build();

    while world.read_resource::<GameFlow>().state != GameState::Exit {
        dispatcher.dispatch(&world);
        // some sleep?
    }

    Ok(())
}
