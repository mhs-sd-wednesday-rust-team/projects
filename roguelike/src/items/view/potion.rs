use std::marker::PhantomData;

use ratatui::{
    style::{Color, Style},
    text::{Span, Text},
};

pub struct PotionView<'a> {
    pub tag: PhantomData<&'a ()>,
    // some stats that affect view
}

impl<'a> From<PotionView<'a>> for Text<'a> {
    fn from(_: PotionView<'a>) -> Self {
        Span::raw("h")
            .style(Style::default().fg(Color::LightMagenta).bg(Color::Black))
            .into()
    }
}
