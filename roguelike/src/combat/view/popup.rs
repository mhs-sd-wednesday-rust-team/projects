use rand::{thread_rng, Rng};
use ratatui::{
    layout::{Constraint, Direction, Flex, Layout},
    style::{Style, Stylize},
    widgets::{Block, Clear, Paragraph, Widget},
};
use specs::{Join, World, WorldExt};

use crate::{
    combat::{CombatFlowState, CombatState},
    monster::view::monster::MonsterView,
    player::{view::player::PlayerView, Player},
};

pub struct CombatFlowView<'a> {
    pub world: &'a World,
}

impl<'a> Widget for CombatFlowView<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let combat_state = self.world.read_resource::<CombatState>();
        let CombatState::Combat(ref combat_state) = *combat_state else {
            return;
        };

        let players = self.world.read_storage::<Player>();
        let entities = self.world.entities();

        let (_, player_entity) = (&players, &entities)
            .join()
            .next()
            .expect("should be a player");

        let is_attacking = player_entity == combat_state.attacker;
        let monster_entity = if is_attacking {
            combat_state.defending
        } else {
            combat_state.attacker
        };

        let vertical = Layout::vertical([Constraint::Length(5)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Length(30)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);

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
                Constraint::Length(4),
                Constraint::Fill(1),
                Constraint::Length(4),
                Constraint::Length(2),
                Constraint::Length(2),
            ])
            .flex(Flex::Center)
            .split(area);

        Paragraph::new("⚔️").render(combat_layout[1], buf);
        Paragraph::new("🛡️").render(combat_layout[5], buf);

        Paragraph::new(PlayerView::default())
            .render(combat_layout[if is_attacking { 0 } else { 6 }], buf);
        Paragraph::new(MonsterView {
            world: self.world,
            entity: monster_entity,
        })
        .render(combat_layout[if is_attacking { 6 } else { 0 }], buf);

        let mut rng = thread_rng();

        let (l_num, r_num) = match combat_state.state {
            CombatFlowState::Tossing => {
                let random_left: i64 = rng.gen_range(0..=8);
                let random_right: i64 = rng.gen_range(0..=8);
                (random_left, random_right)
            }
            CombatFlowState::Tossed {
                attacker_score,
                defending_score,
            } => (attacker_score, defending_score),
            CombatFlowState::HpDiff { defending_diff } => (0, defending_diff),
        };

        Paragraph::new(format!("{:4> }", l_num)).render(combat_layout[2], buf);
        Paragraph::new(format!("{:4> }", r_num)).render(combat_layout[5], buf);
    }
}
