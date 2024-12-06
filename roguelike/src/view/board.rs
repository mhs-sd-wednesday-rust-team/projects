use ratatui::{
    layout::Constraint,
    style::Style,
    widgets::{Cell, Row, Table, Widget},
};

use crate::{
    board::{board::Board, position::Position, tile::Tile},
    player::Player,
};

use super::view_tile::ViewTile;

pub struct BoardView<'a> {
    pub tiles: &'a [&'a [ViewTile<'a>]],
}

impl<'a> Widget for BoardView<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let mut rows = vec![];
        for table_row in self.tiles.iter() {
            let mut cells = vec![];
            for table_cell in table_row.iter() {
                cells.push(Cell::new(table_cell.clone().clone()));
            }
            rows.push(Row::new(cells));
        }

        let widths = vec![Constraint::Length(1); self.tiles[0].len()];

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
