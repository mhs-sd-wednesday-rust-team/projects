use ratatui::{
    style::{Color, Style},
    text::{Span, Text},
};

use crate::monster::{MobStrategy, Monster};

#[derive(Clone)]
pub struct MonsterView<'a> {
    pub monster: &'a Monster,
    pub is_splitting: bool,
}

impl<'a> From<MonsterView<'a>> for Text<'a> {
    fn from(value: MonsterView<'a>) -> Self {
        let glyph = if value.is_splitting {
            "💩"
        } else {
            match value.monster.strategy {
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
