use crossterm::event::{self, KeyCode};
use specs::{DispatcherBuilder, Join, World};
use view::{GameView, PlayView};

use crate::{
    board::{board::Board, position::Position, tile::Tile, WorldTileMapResource}, player::Player, term::{Term, TermEvents}, view::view_tile::ViewTile
};

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
    type SystemData = (
        specs::Read<'a, TermEvents>,
        specs::Write<'a, GameFlow>,
        specs::Write<'a, WorldTileMapResource>
    );

    fn run(&mut self, (term_events, mut game_flow, mut map): Self::SystemData) {
        for event in term_events.0.iter() {
            if let event::Event::Key(k) = event {
                if k.kind == event::KeyEventKind::Release {
                    continue;
                }

                if k.code == KeyCode::Char('q') {
                    game_flow.state = GameState::Exit;
                    return;
                }

                match game_flow.state {
                    GameState::Start => {
                        game_flow.state = GameState::Running
                    },
                    GameState::Running => {},
                    GameState::Finished => {
                        game_flow.state = GameState::Running
                    },
                    GameState::Exit => {},
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
