use crossterm::event::{Event, KeyEventKind};
use specs::{DispatcherBuilder, World};

use crate::term::TermEvents;

#[derive(PartialEq, Eq, Clone, Default)]
pub enum Turn {
    #[default]
    Player,
    Game,
}

struct TurnSystem;

impl<'a> specs::System<'a> for TurnSystem {
    type SystemData = (specs::Read<'a, TermEvents>, specs::Write<'a, Turn>);

    fn run(&mut self, (events, mut turn): Self::SystemData) {
        let has_actions = events
            .0
            .iter()
            .any(|event| matches!(event, Event::Key(k) if k.kind == KeyEventKind::Press));

        let curr_turn = turn.clone();
        *turn = match curr_turn {
            Turn::Game => Turn::Player,
            // todo: check user commands
            Turn::Player if has_actions => Turn::Game,
            Turn::Player => Turn::Player,
        }
    }
}

pub fn register(dispatcher: &mut DispatcherBuilder, world: &mut World) -> anyhow::Result<()> {
    world.insert(Turn::Player);
    dispatcher.add(TurnSystem, "turn_system", &["death_system"]);
    Ok(())
}
