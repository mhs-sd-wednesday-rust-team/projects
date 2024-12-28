use ratatui::{
    layout::{Constraint, Direction, Flex, Layout},
    widgets::{Block, Paragraph, Widget},
};
use specs::{World, WorldExt};

use crate::{
    board::view::board::BoardView, combat::view::bar::CombatBarView,
    experience::view::bar::ExperienceBarView, flow::GameFlow,
};

pub struct PlayView<'a> {
    pub world: &'a World,
}

impl<'a> Widget for PlayView<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let game_flow = self.world.read_resource::<GameFlow>();

        Block::bordered()
            .border_type(ratatui::widgets::BorderType::Thick)
            .title_bottom(format!("level: {}", game_flow.level.as_number()))
            .render(area, buf);

        let center_area = area.inner(ratatui::layout::Margin {
            horizontal: 2,
            vertical: 2,
        });

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Fill(1),
                Constraint::Length(2),
                Constraint::Length(2),
            ])
            .flex(Flex::Center)
            .split(center_area);

        BoardView::new(self.world).render(layout[0], buf);

        let hud_layout = Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(16)
            .constraints(vec![
                Constraint::Length(ExperienceBarView::MIN_LEN),
                Constraint::Length(1),
                Constraint::Length(CombatBarView::MIN_LEN),
                Constraint::Fill(1),
            ])
            .split(layout[1]);

        ExperienceBarView { world: self.world }.render(hud_layout[0], buf);
        CombatBarView { world: self.world }.render(hud_layout[2], buf);

        let hint_layout = Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(16)
            .constraints(vec![Constraint::Fill(1)])
            .split(layout[2]);

        Paragraph::new("move with `arrows` or (`h`,`j`,`k`,`l`); simulate death with `d`")
            .render(hint_layout[0], buf);
    }
}
