use ratatui::{
    layout::Constraint,
    style::Style,
    widgets::{Cell, Row, Table, Widget},
};
use specs::Join;

use crate::{
    board::WorldTileMap,
    components::Position,
    items::{view::potion::PotionView, Potion},
    monster::{view::monster::MonsterView, Monster},
    player::{view::player::PlayerView, Player},
};

use super::view_tile::WorldTile;

pub struct BoardView<'a> {
    board: Vec<Row<'a>>,
    width: usize,
}

impl<'a> BoardView<'a> {
    pub fn new(
        map: specs::Read<'a, WorldTileMap>,
        pos: specs::ReadStorage<'a, Position>,
        player: specs::ReadStorage<'a, Player>,
        monsters: specs::ReadStorage<'a, Monster>,
        potions: specs::ReadStorage<'a, Potion>,
    ) -> Self {
        let mut rows = vec![];
        for board_row in map.board.iter() {
            let mut cells = vec![];
            for board_cell in board_row.iter() {
                cells.push(Cell::new(WorldTile {
                    tile: board_cell.clone(),
                    biome: map.biome.clone(),
                }));
            }
            rows.push(cells);
        }

        for (_, pos) in (&monsters, &pos).join() {
            rows[pos.y as usize][pos.x as usize] = Cell::new(MonsterView::default());
        }
        for (_, pos) in (&potions, &pos).join() {
            rows[pos.y as usize][pos.x as usize] = Cell::new(PotionView::default());
        }
        for (_, pos) in (&player, &pos).join() {
            rows[pos.y as usize][pos.x as usize] = Cell::new(PlayerView::default());
        }

        Self {
            board: rows.drain(..).map(Row::new).collect(),
            width: map.width,
        }
    }
}

impl<'a> Widget for BoardView<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let widths = vec![Constraint::Length(1); self.width];

        Table::new(self.board, widths)
            .style(
                Style::default()
                    .bg(ratatui::style::Color::Reset)
                    .fg(ratatui::style::Color::Reset),
            )
            .column_spacing(0)
            .render(area, buf);
    }
}
