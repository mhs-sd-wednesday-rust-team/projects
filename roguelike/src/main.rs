use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use flow::{GameFlow, GameState};
use ratatui::{
    layout::Alignment,
    prelude::{CrosstermBackend, Terminal},
    style::{Color, Style, Stylize},
    symbols::Marker,
    widgets::{canvas::Canvas, Block, Widget},
};
use specs::{DispatcherBuilder, World, WorldExt};
use std::io::{stdout, Result};
use term::TermEvents;
use tui_big_text::{BigText, PixelSize};

mod board;
mod flow;
mod term;

fn main() -> Result<()> {
    let mut world = World::new();
    let mut dispatcher_builder = DispatcherBuilder::new();

    term::register(&mut dispatcher_builder, &mut world).unwrap();
    board::register(&mut dispatcher_builder, &mut world).unwrap();
    flow::register(&mut dispatcher_builder, &mut world).unwrap();

    let mut dispatcher = dispatcher_builder.build();

    while world.read_resource::<GameFlow>().state != GameState::Exit {
        dispatcher.dispatch(&world);
        // some sleep?
    }

    Ok(())
}
