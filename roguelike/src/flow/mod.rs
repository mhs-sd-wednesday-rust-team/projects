use crossterm::event::{self, KeyCode, KeyEventKind};
use specs::{DispatcherBuilder, Join, World};
use view::{GameView, PlayView};

use crate::{
    board::{position::Position, tile::Tile, WorldTileMap},
    term::{Term, TermEvents},
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

struct RenderSystem;

impl<'a> specs::System<'a> for RenderSystem {
    type SystemData = (
        specs::Write<'a, Term>,
        specs::Read<'a, GameFlow>,
        specs::Read<'a, WorldTileMap>,
        specs::ReadStorage<'a, Position>,
        specs::ReadStorage<'a, Tile>,
    );

    fn run(&mut self, (mut term, game_flow, map, pos, tile): Self::SystemData) {
        term.0
            .draw(|frame| {
                let area = frame.area();
                match game_flow.state {
                    GameState::Start => {
                        frame.render_widget(GameView::Start(view::StartMenuView), area)
                    }
                    GameState::Finished => {
                        frame.render_widget(GameView::Finish(view::FinishMenuView), area)
                    }
                    GameState::Running => {
                        // CoW
                        let mut map = map.clone();

                        for (pos, tile) in (&pos, &tile).join() {
                            // FIXME: handle x,y overflow
                            *map.board
                                .get_mut(pos.y as usize)
                                .unwrap()
                                .get_mut(pos.x as usize)
                                .unwrap() = tile.clone();
                        }

                        frame.render_widget(
                            GameView::Play(PlayView {
                                map: &map,
                                level: &game_flow.level,
                            }),
                            area,
                        )
                    }
                    GameState::Exit => {}
                };
            })
            .unwrap();
    }
}

pub fn register(dispatcher: &mut DispatcherBuilder, world: &mut World) -> anyhow::Result<()> {
    world.insert(GameFlow::default());

    dispatcher.add_thread_local(RenderSystem);

    dispatcher.add(DummyFlowSystem, "dummy_flow", &[]);

    Ok(())
}
