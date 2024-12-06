use ratatui::{
    layout::Constraint,
    style::Style,
    widgets::{Cell, Row, Table, Widget},
};

use crate::board::WorldTileMap;

use super::view_tile::ViewTile;

pub struct BoardView<'a> {
    pub map: &'a WorldTileMap,
}

impl<'a> Widget for BoardView<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let mut rows = vec![];
        for board_row in self.map.board.iter() {
            let mut cells = vec![];
            for board_cell in board_row.iter() {
                cells.push(Cell::new(ViewTile {
                    tile: board_cell,
                    biome: &self.map.biome,
                }));
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
