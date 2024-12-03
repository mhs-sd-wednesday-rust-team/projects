use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Paragraph, Widget},
};

use crate::{
    flow::Level,
    view::{board::BoardView, view_tile::ViewTile},
};

pub struct PlayView<'a> {
    pub level: &'a Level,
    pub tiles: &'a [&'a [ViewTile<'a>]],
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
            .constraints(vec![Constraint::Fill(1), Constraint::Length(2)])
            .split(center_area);

        BoardView { tiles: self.tiles }.render(layout[0], buf);
        Paragraph::new("some hero stats").render(layout[1], buf);
    }
}
