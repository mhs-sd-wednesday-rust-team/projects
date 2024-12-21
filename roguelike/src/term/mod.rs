use std::{
    io::{stdout, Stdout},
    panic::{set_hook, take_hook},
};

use crossterm::{
    event::{self, Event},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::CrosstermBackend, Terminal};
use specs::{DispatcherBuilder, World};

pub struct Term(pub Terminal<CrosstermBackend<Stdout>>);

impl Default for Term {
    fn default() -> Self {
        init_panic_hook();
        stdout().execute(EnterAlternateScreen).unwrap();
        enable_raw_mode().unwrap();
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
        terminal.clear().unwrap();
        Self(terminal)
    }
}

impl Drop for Term {
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

#[derive(Default)]
pub struct TermEvents(pub Vec<Event>);

struct InputSystem;

impl<'a> specs::System<'a> for InputSystem {
    type SystemData = (specs::Write<'a, TermEvents>,);

    fn run(&mut self, mut events: Self::SystemData) {
        events.0 .0.clear();
        if event::poll(std::time::Duration::from_millis(16)).unwrap() {
            let event = event::read().unwrap();
            events.0 .0.push(event);
        }
    }
}

pub fn register(dispatcher: &mut DispatcherBuilder, world: &mut World) -> anyhow::Result<()> {
    world.insert(Term::default());
    world.insert(TermEvents::default());

    dispatcher.add_thread_local(InputSystem);

    Ok(())
}
