use ratatui::{
    style::{Color, Style},
    text::{Span, Text},
};
use specs::{Entity, World, WorldExt};

use crate::monster::{split_ability::SplitMonsterAbility, MobStrategy, Monster};

#[derive(Clone)]
pub struct MonsterView<'a> {
    pub world: &'a World,
    pub entity: Entity,
}

impl<'a> From<MonsterView<'a>> for Text<'a> {
    fn from(value: MonsterView<'a>) -> Self {
        let splitting_store = value.world.read_storage::<SplitMonsterAbility>();
        let monsters_store = value.world.read_storage::<Monster>();

        let is_splitting = splitting_store.contains(value.entity);
        let monster = monsters_store.get(value.entity).unwrap();

        let glyph = if is_splitting {
            "💩"
        } else {
            match monster.strategy {
                MobStrategy::Random => "🌪️",
                MobStrategy::Coward => "🐞",
                MobStrategy::Aggressive => "👾",
            }
        };

        Span::raw(glyph)
            .style(Style::default().fg(Color::LightRed).bg(Color::Black))
            .into()
    }
}
