use std::marker::PhantomData;

use ratatui::{
    style::{Color, Style},
    text::{Span, Text},
};

pub struct MonsterView<'a> {
    pub tag: PhantomData<&'a ()>,
    // some stats that affect view
}

impl<'a> From<MonsterView<'a>> for Text<'a> {
    fn from(_: MonsterView<'a>) -> Self {
        Span::raw("m")
            .style(Style::default().fg(Color::LightRed).bg(Color::Black))
            .into()
    }
}
