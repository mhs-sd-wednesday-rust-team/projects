use ratatui::{
    style::{Color, Style},
    text::{Span, Text},
};

use crate::board::tile::{BiomeKind, Tile, TileKind};

#[derive(Clone)]
pub struct TileView<'a> {
    pub tile: &'a Tile,
}

impl<'a> From<TileView<'a>> for Text<'a> {
    fn from(value: TileView<'a>) -> Self {
        let (glyph, bg, fg) = match value.tile.biome {
            BiomeKind::Ocean => match value.tile.kind {
                TileKind::Ground => ("%", Color::LightBlue, Color::Blue),
                TileKind::Wall => ("@", Color::LightBlue, Color::Red),
            },
            BiomeKind::Beach => match value.tile.kind {
                TileKind::Ground => ("#", Color::LightYellow, Color::Yellow),
                TileKind::Wall => ("$", Color::LightYellow, Color::Black),
            },
        };

        Span::raw(glyph)
            .style(Style::default().fg(fg).bg(bg))
            .into()
    }
}
