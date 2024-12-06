use crossterm::event::{self, KeyCode, KeyEventKind};
use specs::{DispatcherBuilder, World};

use crate::term::TermEvents;

pub mod view;

#[derive(PartialEq, Eq)]
pub enum GameState {
    Start,
    Running,
    Finished,
    Exit,
}

pub struct Level(usize);

impl Level {
    pub fn as_number(&self) -> usize {
        self.0 + 1
    }
}

pub struct GameFlow {
    pub state: GameState,
    pub level: Level,
}

impl GameFlow {
    pub fn new() -> Self {
        Self {
            state: GameState::Start,
            level: Level(0),
        }
    }
}

impl Default for GameFlow {
    fn default() -> Self {
        Self::new()
    }
}

struct DummyFlowSystem;

impl<'a> specs::System<'a> for DummyFlowSystem {
    type SystemData = (specs::Read<'a, TermEvents>, specs::Write<'a, GameFlow>);

    fn run(&mut self, (term_events, mut game_flow): Self::SystemData) {
        for event in term_events.0.iter() {
            if let event::Event::Key(k) = event {
                if k.kind == KeyEventKind::Press {
                    match k.code {
                        KeyCode::Char('1') => game_flow.state = GameState::Start,
                        KeyCode::Char('2') => game_flow.state = GameState::Running,
                        KeyCode::Char('3') => game_flow.state = GameState::Finished,
                        KeyCode::Char('q') => game_flow.state = GameState::Exit,
                        _ => {}
                    }
                }
            }
        }
    }
}

pub fn register(dispatcher: &mut DispatcherBuilder, world: &mut World) -> anyhow::Result<()> {
    world.insert(GameFlow::default());

    dispatcher.add(DummyFlowSystem, "dummy_flow", &[]);

    Ok(())
}
