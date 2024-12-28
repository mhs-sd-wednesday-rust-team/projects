#![allow(clippy::needless_lifetimes)]
use specs::{DispatcherBuilder, World, WorldExt};
use term::terminal::CrosstermApp;

mod board;
mod combat;
mod components;
mod experience;
mod flow;
mod items;
mod monster;
mod movement;
mod player;
mod term;
mod turn;

fn main() -> anyhow::Result<()> {
    let mut world = World::new();
    let mut dispatcher_builder = DispatcherBuilder::new();

    term::register(&mut dispatcher_builder, &mut world)?;
    board::register(&mut dispatcher_builder, &mut world)?;
    items::register(&mut dispatcher_builder, &mut world)?;
    movement::register(&mut dispatcher_builder, &mut world)?;
    player::register(&mut dispatcher_builder, &mut world)?;
    monster::register(&mut dispatcher_builder, &mut world)?;
    combat::register(&mut dispatcher_builder, &mut world)?;
    experience::register(&mut dispatcher_builder, &mut world)?;
    turn::register(&mut dispatcher_builder, &mut world)?;
    flow::register(&mut dispatcher_builder, &mut world)?;

    let dispatcher = dispatcher_builder.build();

    let app = CrosstermApp::new(world, dispatcher);
    app.run()?;

    Ok(())
}
