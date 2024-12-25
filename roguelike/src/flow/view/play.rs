use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Paragraph, Widget},
};

use crate::{
    board::view::board::BoardView,
    experience::{view::bar::ExperienceBarView, Experience},
    flow::Level,
};

pub struct PlayView<'a> {
    pub level: &'a Level,
    pub player_experience: &'a Experience,
    pub board: BoardView<'a>,
}

impl<'a> Widget for PlayView<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Block::bordered()
            .border_type(ratatui::widgets::BorderType::Thick)
            .title_bottom(format!("level: {}", self.level.as_number()))
            .render(area, buf);

        let center_area = area.inner(ratatui::layout::Margin {
            horizontal: 2,
            vertical: 2,
        });

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Fill(1), Constraint::Length(4)])
            .split(center_area);

        self.board.render(layout[0], buf);

        let hud_layout = Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(16)
            .constraints(vec![
                Constraint::Length(ExperienceBarView::MIN_LEN),
                Constraint::Fill(1),
                Constraint::Length(100),
            ])
            .split(layout[1]);

        ExperienceBarView {
            experience: self.player_experience,
        }
        .render(hud_layout[0], buf);

        Paragraph::new("move with `arrows` or (`h`,`j`,`k`,`l`); simulate death with `d`")
            .render(hud_layout[2], buf);
    }
}
