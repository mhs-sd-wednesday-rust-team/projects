use crossterm::event::{Event, KeyCode, KeyEventKind};
use specs::prelude::*;
use specs::{Component, DenseVecStorage, DispatcherBuilder, World, WorldExt};

use crate::flow::{GameFlow, GameState};
use crate::movement::MoveAction;
use crate::term::TermEvents;

pub mod view;

#[derive(Component)]
pub struct Player {}

struct PlayerMoveSystem;

impl<'a> specs::System<'a> for PlayerMoveSystem {
    type SystemData = (
        specs::Entities<'a>,
        specs::WriteStorage<'a, MoveAction>,
        specs::WriteStorage<'a, Player>,
        specs::Read<'a, TermEvents>,
        specs::Read<'a, GameFlow>,
    );

    fn run(&mut self, (entities, mut moves, players, term_events, game_flow): Self::SystemData) {
        let GameState::Running = game_flow.state else {
            return;
        };

        for event in term_events.0.iter() {
            if let Event::Key(k) = event {
                if k.kind == KeyEventKind::Press {
                    let (delta_x, delta_y) = match k.code {
                        KeyCode::Up | KeyCode::Char('k') => (0, -1),
                        KeyCode::Down | KeyCode::Char('j') => (0, 1),
                        KeyCode::Left | KeyCode::Char('h') => (-1, 0),
                        KeyCode::Right | KeyCode::Char('l') => (1, 0),
                        _ => continue,
                    };

                    for (_, e) in (&players, &entities).join() {
                        moves.insert(e, MoveAction::new(delta_x, delta_y)).unwrap();
                    }
                }
            }
        }
    }
}

pub fn register(dispatcher: &mut DispatcherBuilder, world: &mut World) -> anyhow::Result<()> {
    world.register::<Player>();

    dispatcher.add(PlayerMoveSystem, "player_move_system", &[]);
    Ok(())
}
