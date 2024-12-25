use ratatui::{
    style::{Color, Style},
    text::{Span, Text},
};

use crate::board::tile::{Biome, Tile};

#[allow(unused)]
pub trait TileView<'a>: Into<Text<'a>> {}

#[derive(Clone)]
pub struct WorldTile {
    pub tile: Tile,
    pub biome: Biome,
}

impl<'a> From<WorldTile> for Text<'a> {
    fn from(value: WorldTile) -> Self {
        let (glyph, bg, fg) = match value.tile {
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

impl<'a, T> TileView<'a> for T where T: Into<Text<'a>> {}
