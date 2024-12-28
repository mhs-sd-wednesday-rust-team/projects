use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{LineGauge, Paragraph, Widget},
};
use specs::{Join, World, WorldExt};

use crate::{experience::Experience, player::Player};

pub struct ExperienceBarView<'a> {
    pub world: &'a World,
}

impl<'a> ExperienceBarView<'a> {
    pub const MIN_LEN: u16 = 25;
}

impl<'a> Widget for ExperienceBarView<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let players = self.world.read_storage::<Player>();
        let experiences = self.world.read_storage::<Experience>();

        let (_, player_experience) = (&players, &experiences)
            .join()
            .next()
            .expect("should be a player");

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
            .ratio(player_experience.exp_ratio())
            .render(exp_layout[1], buf);

        Paragraph::new("]").render(exp_layout[2], buf);

        Paragraph::new(format!("lvl {}", player_experience.level)).render(exp_layout[4], buf);
    }
}
