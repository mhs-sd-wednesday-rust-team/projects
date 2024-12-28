use ratatui::{
    layout::{Constraint, Flex, Layout},
    style::{Color, Style},
    widgets::{Cell, Row, Table, Widget},
};
use specs::Join;

use crate::{
    board::WorldTileMap,
    combat::CombatStats,
    items::{view::potion::PotionView, Potion},
    monster::{split_ability::SplitMonsterAbility, view::monster::MonsterView, Monster},
    movement::Position,
    player::{view::player::PlayerView, Player},
};

use super::view_tile::WorldTile;

pub struct BoardView<'a> {
    board: Vec<Row<'a>>,
    width: usize,
}

pub struct BoardViewContext<'a> {
    pub entities: &'a specs::Entities<'a>,
    pub map: &'a specs::Read<'a, WorldTileMap>,
    pub pos: &'a specs::ReadStorage<'a, Position>,
    pub player: &'a specs::ReadStorage<'a, Player>,
    pub monsters: &'a specs::ReadStorage<'a, Monster>,
    pub stats: &'a specs::ReadStorage<'a, CombatStats>,
    pub splitting_monsters: &'a specs::ReadStorage<'a, SplitMonsterAbility>,
    pub potions: &'a specs::ReadStorage<'a, Potion>,
}

impl<'a> BoardView<'a> {
    pub fn new(ctx: BoardViewContext<'a>) -> Self {
        let mut rows = vec![];
        for board_row in ctx.map.board.iter() {
            let mut cells = vec![];
            for board_cell in board_row.iter() {
                cells.push(Cell::new(WorldTile {
                    tile: board_cell.clone(),
                    biome: ctx.map.biome.clone(),
                }));
            }
            rows.push(cells);
        }

        for (_, pos) in (ctx.potions, ctx.pos).join() {
            rows[pos.y as usize][pos.x as usize] = Cell::new(PotionView::default());
        }
        for (e, monster, pos, stat) in (ctx.entities, ctx.monsters, ctx.pos, ctx.stats).join() {
            rows[pos.y as usize][pos.x as usize] = Cell::new(MonsterView {
                monster,
                is_splitting: ctx.splitting_monsters.contains(e),
            });
            if pos.y > 0 {
                rows[pos.y as usize - 1][pos.x as usize] =
                    Cell::new(format!("{:2> }", stat.hp)).style(Style::default().fg(Color::Red));
            }
        }
        for (_, pos) in (ctx.player, ctx.pos).join() {
            rows[pos.y as usize][pos.x as usize] = Cell::new(PlayerView::default());
        }

        Self {
            board: rows.drain(..).map(Row::new).collect(),
            width: ctx.map.width,
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
