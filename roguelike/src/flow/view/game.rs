use ratatui::widgets::Widget;

use super::{finish_menu::FinishMenuView, play::PlayView, start_menu::StartMenuView};

pub enum GameView<'a> {
    Start(StartMenuView),
    Finish(FinishMenuView),
    Play(PlayView<'a>),
}

impl<'a> Widget for GameView<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        match self {
            Self::Start(m) => m.render(area, buf),
            Self::Finish(m) => m.render(area, buf),
            Self::Play(m) => m.render(area, buf),
        }
    }
}
