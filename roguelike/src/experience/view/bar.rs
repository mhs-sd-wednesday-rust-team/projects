use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{LineGauge, Paragraph, Widget},
};

use crate::experience::Experience;

pub struct ExperienceBarView<'a> {
    pub experience: &'a Experience,
}

impl<'a> ExperienceBarView<'a> {
    pub const MIN_LEN: u16 = 25;
}

impl<'a> Widget for ExperienceBarView<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let exp_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Length(6),
                Constraint::Length(10),
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(7),
            ])
            .split(area);

        Paragraph::new("exp: [").render(exp_layout[0], buf);

        LineGauge::default()
            .ratio(self.experience.exp_ratio())
            .render(exp_layout[1], buf);

        Paragraph::new("]").render(exp_layout[2], buf);

        Paragraph::new(format!("lvl {}", self.experience.level)).render(exp_layout[4], buf);
    }
}
