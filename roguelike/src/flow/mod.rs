use crossterm::event::{self, KeyCode};
use specs::prelude::*;
use specs::{DispatcherBuilder, World};

use crate::player::find_player_spawn_position;
use crate::{
    board::{generator::generate_map, position::Position, WorldTileMap},
    player::Player,
    term::TermEvents,
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
        specs::Write<'a, WorldTileMap>,
        specs::WriteStorage<'a, Position>,
        specs::WriteStorage<'a, Player>,
    );

    fn run(
        &mut self,
        (term_events, mut game_flow, mut tile_map, mut positions, players): Self::SystemData,
    ) {
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
                    GameState::Start => game_flow.state = GameState::Running,
                    GameState::Running => {
                        // FIXME: mock switch to "death".
                        if k.code == KeyCode::Char('d') {
                            game_flow.state = GameState::Finished;
                        }
                    }
                    GameState::Finished => {
                        let map = generate_map();
                        tile_map.set_map(&map);

                        for (_, pos) in (&players, &mut positions).join() {
                            *pos = find_player_spawn_position(&tile_map)
                                .unwrap_or_else(|e| panic!("{e:?}"));
                        }
                        // TODO: Should also reinitialize stats and update monsters.

                        game_flow.state = GameState::Running
                    }
                    GameState::Exit => {}
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
