use rand::{thread_rng, Rng};
use ratatui::{
    layout::{Constraint, Direction, Flex, Layout},
    style::{Style, Stylize},
    widgets::{Block, Clear, Paragraph, Widget},
};

use crate::combat::CombatFlowState;

pub struct CombatFlowView<'a> {
    pub state: &'a CombatFlowState,
    pub is_attacking: bool,
}

impl<'a> Widget for CombatFlowView<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Clear.render(area, buf);

        Block::bordered()
            .border_style(Style::default().blue())
            .title("combat")
            .render(area, buf);

        let combat_layout = Layout::default()
            .direction(Direction::Horizontal)
            .margin(2)
            .constraints(vec![
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(3),
                Constraint::Fill(1),
                Constraint::Length(3),
                Constraint::Length(2),
                Constraint::Length(2),
            ])
            .flex(Flex::Center)
            .split(area);

        Paragraph::new("⚔️").render(combat_layout[1], buf);
        Paragraph::new("🛡️").render(combat_layout[5], buf);

        Paragraph::new("🦀").render(combat_layout[if self.is_attacking { 0 } else { 6 }], buf);
        Paragraph::new("👾").render(combat_layout[if self.is_attacking { 6 } else { 0 }], buf);

        let mut rng = thread_rng();

        let (l_num, r_num) = match self.state {
            CombatFlowState::Tossing => {
                let random_left: i64 = rng.gen_range(0..=8);
                let random_right: i64 = rng.gen_range(0..=8);
                (random_left, random_right)
            }
            CombatFlowState::Tossed {
                attacker_score,
                defending_score,
            } => (*attacker_score, *defending_score),
            CombatFlowState::HpDiff { defending_diff } => (0, *defending_diff),
        };

        Paragraph::new(format!("{:3> }", l_num)).render(combat_layout[2], buf);
        Paragraph::new(format!("{:3> }", r_num)).render(combat_layout[5], buf);
    }
}
