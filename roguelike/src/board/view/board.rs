use ratatui::{
    layout::Constraint,
    style::Style,
    widgets::{Cell, Row, Table, Widget},
};

use crate::{
    board::{position::Position, WorldTileMap},
    monster::view::monster::MonsterView,
    player::view::player::PlayerView,
};

use super::view_tile::WorldTile;

pub struct BoardView<'a> {
    pub map: &'a WorldTileMap,
    pub player_pos: &'a Position,
    pub monsters_pos: Vec<&'a Position>,
}

impl<'a> Widget for BoardView<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let mut rows = vec![];
        for (y, board_row) in self.map.board.iter().enumerate() {
            let mut cells = vec![];
            for (x, board_cell) in board_row.iter().enumerate() {
                if self
                    .monsters_pos
                    .iter()
                    .any(|pos| pos.x == x as i64 && pos.y == y as i64)
                {
                    cells.push(Cell::new(MonsterView {
                        tag: Default::default(),
                    }));
                } else if self.player_pos.x == x as i64 && self.player_pos.y == y as i64 {
                    cells.push(Cell::new(PlayerView {
                        tag: Default::default(),
                    }));
                } else {
                    cells.push(Cell::new(WorldTile {
                        tile: board_cell,
                        biome: &self.map.biome,
                    }));
                };
            }
            rows.push(Row::new(cells));
        }

        let widths = vec![Constraint::Length(1); self.map.width];

        Table::new(rows, widths)
            .style(
                Style::default()
                    .bg(ratatui::style::Color::Reset)
                    .fg(ratatui::style::Color::Reset),
            )
            .column_spacing(0)
            .render(area, buf);
    }
}
