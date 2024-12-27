use std::ops::Deref;

use ratatui::{
    layout::{Constraint, Flex, Layout},
    widgets::Widget,
};
use specs::{prelude::ResourceId, DispatcherBuilder, Join, SystemData, World};

use crate::{
    board::{view::board::BoardView, WorldTileMap},
    combat::{view::popup::CombatFlowView, CombatState, CombatStats},
    experience::Experience,
    flow::{
        view::{FinishMenuView, GameView, PlayView, StartMenuView},
        GameFlow, GameState,
    },
    items::Potion,
    monster::Monster,
    movement::Position,
    player::Player,
    term::Term,
};

struct RenderSystem;

#[derive(specs::SystemData)]
struct RenderSystemData<'a> {
    entities: specs::Entities<'a>,
    term: specs::Write<'a, Term>,
    game_flow: specs::Read<'a, GameFlow>,
    combat: specs::Read<'a, CombatState>,
    map: specs::Read<'a, WorldTileMap>,
    pos: specs::ReadStorage<'a, Position>,
    player: specs::ReadStorage<'a, Player>,
    monsters: specs::ReadStorage<'a, Monster>,
    stats: specs::ReadStorage<'a, CombatStats>,
    potions: specs::ReadStorage<'a, Potion>,
    experience: specs::ReadStorage<'a, Experience>,
}

struct RenderView<'a> {
    pub entities: &'a specs::Entities<'a>,
    pub game_flow: &'a specs::Read<'a, GameFlow>,
    pub combat: &'a specs::Read<'a, CombatState>,
    pub map: &'a specs::Read<'a, WorldTileMap>,
    pub pos: &'a specs::ReadStorage<'a, Position>,
    pub player: &'a specs::ReadStorage<'a, Player>,
    pub monsters: &'a specs::ReadStorage<'a, Monster>,
    pub stats: &'a specs::ReadStorage<'a, CombatStats>,
    pub potions: &'a specs::ReadStorage<'a, Potion>,
    pub experience: &'a specs::ReadStorage<'a, Experience>,
}

impl<'a> Widget for RenderView<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        match self.game_flow.state {
            GameState::Start => GameView::Start(StartMenuView).render(area, buf),
            GameState::Finished => GameView::Finish(FinishMenuView).render(area, buf),
            GameState::Running | GameState::Combat => {
                let (_, player_experience, player_stats, player_entity) =
                    (self.player, self.experience, self.stats, self.entities)
                        .join()
                        .next()
                        .expect("should be a player");

                let board = BoardView::new(
                    self.map,
                    self.pos,
                    self.player,
                    self.monsters,
                    self.stats,
                    self.potions,
                );

                GameView::Play(PlayView {
                    board,
                    player_experience,
                    player_stats,
                    level: &self.game_flow.level,
                })
                .render(area, buf);

                if let CombatState::Combat(ref state) = self.combat.deref() {
                    let vertical = Layout::vertical([Constraint::Length(5)]).flex(Flex::Center);
                    let horizontal =
                        Layout::horizontal([Constraint::Length(30)]).flex(Flex::Center);
                    let [area] = vertical.areas(area);
                    let [area] = horizontal.areas(area);

                    CombatFlowView {
                        state: &state.state,
                        is_attacking: player_entity == state.attacker,
                    }
                    .render(area, buf);
                }
            }
            GameState::Exit => {}
        };
    }
}

impl<'a> specs::System<'a> for RenderSystem {
    type SystemData = RenderSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        data.term
            .0
            .draw(|frame| {
                let area = frame.area();
                frame.render_widget(
                    RenderView {
                        combat: &data.combat,
                        entities: &data.entities,
                        map: &data.map,
                        pos: &data.pos,
                        stats: &data.stats,
                        player: &data.player,
                        potions: &data.potions,
                        monsters: &data.monsters,
                        game_flow: &data.game_flow,
                        experience: &data.experience,
                    },
                    area,
                );
            })
            .unwrap();
    }
}

pub fn register(dispatcher: &mut DispatcherBuilder, _: &mut World) -> anyhow::Result<()> {
    dispatcher.add_thread_local(RenderSystem);
    Ok(())
}
