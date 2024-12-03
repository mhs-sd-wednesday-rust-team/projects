use ratatui::{
    style::{Color, Style},
    text::{Span, Text},
};

use crate::board::tile::{BiomeKind, Tile, TileKind};

#[derive(Clone)]
pub enum ViewTile<'a> {
    WorldTile(&'a Tile),
    PlayerTile,
}

impl<'a> From<ViewTile<'a>> for Text<'a> {
    fn from(value: ViewTile<'a>) -> Self {
        match value {
            ViewTile::WorldTile(tile) => render_world_tile(tile),
            ViewTile::PlayerTile => render_player_tile(),
        }
    }
}

fn render_world_tile(tile: &Tile) -> Text {
    let (glyph, bg, fg) = match tile.biome {
        BiomeKind::Ocean => match tile.kind {
            TileKind::Ground => ("%", Color::LightBlue, Color::Blue),
            TileKind::Wall => ("@", Color::LightBlue, Color::Red),
        },
        BiomeKind::Beach => match tile.kind {
            TileKind::Ground => ("#", Color::LightYellow, Color::Yellow),
            TileKind::Wall => ("$", Color::LightYellow, Color::Black),
        },
        BiomeKind::Castle => match tile.kind {
            TileKind::Ground => (".", Color::Black, Color::Gray),
            TileKind::Wall => ("#", Color::Black, Color::White),
        },
    };

    Span::raw(glyph)
        .style(Style::default().fg(fg).bg(bg))
        .into()
}


fn render_player_tile() -> Text<'static> {
    Span::raw("@")
        .style(Style::default().fg(Color::LightYellow).bg(Color::Black))
        .into()
}
