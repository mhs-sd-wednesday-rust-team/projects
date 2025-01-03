#![allow(clippy::needless_lifetimes)]
use flow::{GameFlow, GameState};
use specs::{DispatcherBuilder, World, WorldExt};
use std::io::Result;

mod board;
mod combat;
mod components;
mod experience;
mod flow;
mod items;
mod monster;
mod player;
mod render;
mod term;
mod turn;

fn main() -> Result<()> {
    let mut world = World::new();
    let mut dispatcher_builder = DispatcherBuilder::new();

    term::register(&mut dispatcher_builder, &mut world).unwrap();
    board::register(&mut dispatcher_builder, &mut world).unwrap();
    items::register(&mut dispatcher_builder, &mut world).unwrap();
    player::register(&mut dispatcher_builder, &mut world).unwrap();
    monster::register(&mut dispatcher_builder, &mut world).unwrap();
    combat::register(&mut dispatcher_builder, &mut world).unwrap();
    experience::register(&mut dispatcher_builder, &mut world).unwrap();
    turn::register(&mut dispatcher_builder, &mut world).unwrap();
    flow::register(&mut dispatcher_builder, &mut world).unwrap();
    render::register(&mut dispatcher_builder, &mut world).unwrap();

    let mut dispatcher = dispatcher_builder.build();

    while world.read_resource::<GameFlow>().state != GameState::Exit {
        dispatcher.dispatch(&world);
        world.maintain();
        // some sleep?
    }

    Ok(())
}
