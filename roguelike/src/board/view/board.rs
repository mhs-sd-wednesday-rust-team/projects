use ratatui::{
    layout::{Constraint, Flex, Layout},
    style::{Color, Style},
    widgets::{Cell, Row, Table, Widget},
};
use specs::Join;

use crate::{
    board::WorldTileMap,
    combat::CombatStats,
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
        map: &specs::Read<'a, WorldTileMap>,
        pos: &specs::ReadStorage<'a, Position>,
        player: &specs::ReadStorage<'a, Player>,
        monsters: &specs::ReadStorage<'a, Monster>,
        stats: &specs::ReadStorage<'a, CombatStats>,
        potions: &specs::ReadStorage<'a, Potion>,
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

        for (_, pos) in (potions, pos).join() {
            rows[pos.y as usize][pos.x as usize] = Cell::new(PotionView::default());
        }
        for (_, pos, stat) in (monsters, pos, stats).join() {
            rows[pos.y as usize][pos.x as usize] = Cell::new(MonsterView::default());
            if pos.y > 0 {
                rows[pos.y as usize - 1][pos.x as usize] =
                    Cell::new(format!("{:2> }", stat.hp)).style(Style::default().fg(Color::Red));
            }
        }
        for (_, pos) in (player, pos).join() {
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
        let [area] = Layout::horizontal([Constraint::Length(2 * self.width as u16)])
            .flex(Flex::Center)
            .areas(area);
        let [area] = Layout::vertical([Constraint::Fill(self.board.len() as u16)])
            .flex(Flex::Center)
            .areas(area);

        let widths = vec![Constraint::Length(2); self.width];
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
