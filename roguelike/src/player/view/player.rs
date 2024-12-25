use std::marker::PhantomData;

use ratatui::{
    style::{Color, Style},
    text::{Span, Text},
};

#[derive(Default)]
pub struct PlayerView<'a> {
    pub tag: PhantomData<&'a ()>,
    // some stats that affect view
}

impl<'a> From<PlayerView<'a>> for Text<'a> {
    fn from(_: PlayerView<'a>) -> Self {
        Span::raw("@")
            .style(Style::default().fg(Color::LightYellow).bg(Color::Black))
            .into()
    }
}
