use ratatui::{
    style::{Color, Style},
    text::{Span, Text},
};

use crate::board::tile::{Biome, Tile};

#[derive(Clone)]
pub struct ViewTile<'a> {
    pub tile: &'a Tile,
    pub biome: &'a Biome,
}

impl<'a> From<ViewTile<'a>> for Text<'a> {
    fn from(value: ViewTile<'a>) -> Self {
        let (glyph, bg, fg) = match value.tile {
            Tile::Player => ("@", Color::LightYellow, Color::Black),
            Tile::Wall => match value.biome {
                Biome::Ocean => ("@", Color::LightBlue, Color::Red),
                Biome::Beach => ("$", Color::LightYellow, Color::Black),
                Biome::Castle => ("#", Color::Black, Color::White),
            },
            Tile::Ground => match value.biome {
                Biome::Ocean => ("%", Color::LightBlue, Color::Blue),
                Biome::Beach => ("#", Color::LightYellow, Color::Yellow),
                Biome::Castle => (".", Color::Black, Color::Gray),
            },
        };

        Span::raw(glyph)
            .style(Style::default().fg(fg).bg(bg))
            .into()
    }
}
