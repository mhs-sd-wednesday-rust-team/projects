use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Paragraph, Widget},
};

use crate::{
    board::{view::board::BoardView, WorldTileMap},
    components::Position,
    flow::Level,
};

pub struct PlayView<'a> {
    pub level: &'a Level,
    pub map: &'a WorldTileMap,
    pub player: &'a Position,
    pub monsters: Vec<&'a Position>,
    pub potions: Vec<&'a Position>,
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

        BoardView {
            map: self.map,
            player_pos: self.player,
            monsters_pos: self.monsters,
            potion_pos: self.potions,
        }
        .render(layout[0], buf);
        Paragraph::new("move with `arrows` or (`h`,`j`,`k`,`l`); simulate death with `d`")
            .render(layout[1], buf);
    }
}
