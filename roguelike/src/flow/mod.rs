use crossterm::event::{Event, KeyCode, KeyEventKind};
use specs::{DispatcherBuilder, Join, World};

use crate::combat::CombatStats;
use crate::player::Player;
use crate::term::TermEvents;

pub mod view;

#[derive(PartialEq, Eq)]
pub enum GameState {
    Start,
    Started,
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

struct GameFlowSystem;

impl<'a> specs::System<'a> for GameFlowSystem {
    type SystemData = (
        specs::Read<'a, TermEvents>,
        specs::ReadStorage<'a, Player>,
        specs::ReadStorage<'a, CombatStats>,
        specs::Write<'a, GameFlow>,
    );

    fn run(&mut self, (term_events, player, stats, mut game_flow): Self::SystemData) {
        match game_flow.state {
            GameState::Start => {
                for event in term_events.0.iter() {
                    if let Event::Key(k) = event {
                        if k.kind == KeyEventKind::Press {
                            if let KeyCode::Char('q') = k.code {
                                game_flow.state = GameState::Exit;
                            } else {
                                game_flow.state = GameState::Started;
                            }
                        }
                    }
                }
            }
            GameState::Started => {
                game_flow.state = GameState::Running;
                for event in term_events.0.iter() {
                    if let Event::Key(k) = event {
                        if k.kind == KeyEventKind::Press {
                            if let KeyCode::Char('q') = k.code {
                                game_flow.state = GameState::Exit;
                            }
                        }
                    }
                }
            }
            GameState::Running => {
                for (player_stats, _) in (&stats, &player).join() {
                    if player_stats.hp <= 0 {
                        game_flow.state = GameState::Finished;
                        return;
                    }
                }
                for event in term_events.0.iter() {
                    if let Event::Key(k) = event {
                        if k.kind == KeyEventKind::Press {
                            if let KeyCode::Char('d') = k.code {
                                game_flow.state = GameState::Finished;
                            }
                        }
                    }
                }
            }
            GameState::Finished => {
                for event in term_events.0.iter() {
                    if let Event::Key(k) = event {
                        if k.kind == KeyEventKind::Press {
                            match k.code {
                                KeyCode::Enter => {
                                    game_flow.state = GameState::Started;
                                }
                                KeyCode::Char('q') => {
                                    game_flow.state = GameState::Exit;
                                }
                                _ => {}
                            };
                        }
                    }
                }
            }
            GameState::Exit => {}
        }
    }
}

pub fn register(dispatcher: &mut DispatcherBuilder, world: &mut World) -> anyhow::Result<()> {
    world.insert(GameFlow::default());

    dispatcher.add(GameFlowSystem, "dummy_flow", &["death_system"]);

    Ok(())
}
