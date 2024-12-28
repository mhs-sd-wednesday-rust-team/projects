use ratatui::widgets::Widget;
use specs::{World, WorldExt};

use crate::flow::{GameFlow, GameState};

use super::{finish_menu::FinishMenuView, play::PlayView, start_menu::StartMenuView};

pub struct GameView<'a> {
    pub world: &'a World,
}

impl<'a> Widget for GameView<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let game_flow = self.world.read_resource::<GameFlow>();

        match game_flow.state {
            GameState::Start => StartMenuView.render(area, buf),
            GameState::Finished => FinishMenuView.render(area, buf),
            GameState::Started => {}
            GameState::Running => {
                PlayView { world: self.world }.render(area, buf);
            }
            GameState::Exit => {}
        };
    }
}
