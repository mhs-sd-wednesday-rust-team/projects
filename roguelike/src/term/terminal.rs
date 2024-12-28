use std::{
    io::{stdout, Stdout},
    panic::{set_hook, take_hook},
};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::CrosstermBackend, Terminal};
use specs::{Dispatcher, World, WorldExt};

use crate::flow::{view::GameView, GameFlow, GameState};

use super::{Command, TermCommands};

pub struct CrosstermApp<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl<'a, 'b> CrosstermApp<'a, 'b> {
    pub fn new(world: World, dispatcher: Dispatcher<'a, 'b>) -> Self {
        init_panic_hook();
        stdout().execute(EnterAlternateScreen).unwrap();
        enable_raw_mode().unwrap();
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
        terminal.clear().unwrap();

        Self {
            world,
            dispatcher,
            terminal,
        }
    }

    pub fn run(mut self) -> Result<()> {
        while self.world.read_resource::<GameFlow>().state != GameState::Exit {
            self.poll()?;

            self.dispatcher.dispatch(&self.world);
            self.world.maintain();

            self.terminal
                .draw(|frame| {
                    let area = frame.area();
                    frame.render_widget(GameView { world: &self.world }, area);
                })
                .unwrap();
        }
        Ok(())
    }

    fn poll(&mut self) -> Result<()> {
        let mut commands_res = self.world.write_resource::<TermCommands>();
        commands_res.0.clear();
        if event::poll(std::time::Duration::from_millis(16)).unwrap() {
            let event = event::read().unwrap();

            let command = if let Event::Key(e) = event {
                if KeyEventKind::Press == e.kind {
                    match e.code {
                        KeyCode::Up | KeyCode::Char('k') => Some(Command::Up),
                        KeyCode::Down | KeyCode::Char('j') => Some(Command::Down),
                        KeyCode::Left | KeyCode::Char('h') => Some(Command::Left),
                        KeyCode::Right | KeyCode::Char('l') => Some(Command::Right),
                        KeyCode::Char('q') => Some(Command::Quit),
                        KeyCode::Char('d') => Some(Command::Death),
                        KeyCode::Enter => Some(Command::Confirm),
                        _ => Some(Command::AnyKey),
                    }
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(cmd) = command {
                commands_res.0.push(cmd);
            }
        }

        Ok(())
    }
}

impl<'a, 'b> Drop for CrosstermApp<'a, 'b> {
    fn drop(&mut self) {
        restore_tui();
    }
}

fn restore_tui() {
    stdout().execute(LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}

fn init_panic_hook() {
    let original_hook = take_hook();
    set_hook(Box::new(move |panic_info| {
        restore_tui();
        original_hook(panic_info);
    }));
}
