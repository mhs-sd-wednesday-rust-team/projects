use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{LineGauge, Paragraph, Widget},
};

use crate::combat::CombatStats;

pub struct CombatBarView<'a> {
    pub stats: &'a CombatStats,
}

impl<'a> CombatBarView<'a> {
    pub const MIN_LEN: u16 = 27;
}

impl<'a> Widget for CombatBarView<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let combat_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Length(5),
                Constraint::Length(10),
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(10),
            ])
            .split(area);

        Paragraph::new("hp: [").render(combat_layout[0], buf);

        LineGauge::default()
            .ratio(self.stats.hp_ratio())
            .render(combat_layout[1], buf);

        Paragraph::new("]").render(combat_layout[2], buf);

        Paragraph::new(format!(
            "{:3> }a, {:3> }d",
            self.stats.power, self.stats.defense
        ))
        .render(combat_layout[4], buf);
    }
}
