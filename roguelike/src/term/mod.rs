use specs::{DispatcherBuilder, World};

pub mod terminal;

pub enum Command {
    Confirm,
    Left,
    Up,
    Right,
    Down,
    Quit,
    Death,
    AnyKey,
}

#[derive(Default)]
pub struct TermCommands(pub Vec<Command>);

pub fn register(_: &mut DispatcherBuilder, world: &mut World) -> anyhow::Result<()> {
    world.insert(TermCommands::default());
    Ok(())
}
