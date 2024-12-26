use specs::{prelude::ResourceId, DispatcherBuilder, Join, SystemData, World};

use crate::{
    board::{view::board::BoardView, WorldTileMap},
    combat::CombatStats,
    components::Position,
    experience::Experience,
    flow::{
        view::{FinishMenuView, GameView, PlayView, StartMenuView},
        GameFlow, GameState,
    },
    items::Potion,
    monster::Monster,
    player::Player,
    term::Term,
};

struct RenderSystem;

#[derive(specs::SystemData)]
struct RenderSystemData<'a> {
    term: specs::Write<'a, Term>,
    game_flow: specs::Read<'a, GameFlow>,
    map: specs::Read<'a, WorldTileMap>,
    pos: specs::ReadStorage<'a, Position>,
    player: specs::ReadStorage<'a, Player>,
    monsters: specs::ReadStorage<'a, Monster>,
    stats: specs::ReadStorage<'a, CombatStats>,
    potions: specs::ReadStorage<'a, Potion>,
    experience: specs::ReadStorage<'a, Experience>,
}

impl<'a> specs::System<'a> for RenderSystem {
    type SystemData = RenderSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        data.term
            .0
            .draw(|frame| {
                let area = frame.area();
                match data.game_flow.state {
                    GameState::Start => frame.render_widget(GameView::Start(StartMenuView), area),
                    GameState::Finished => {
                        frame.render_widget(GameView::Finish(FinishMenuView), area)
                    }
                    GameState::Running => {
                        let (_, player_experience, player_stats) =
                            (&data.player, &data.experience, &data.stats)
                                .join()
                                .next()
                                .expect("should be a player");

                        let board = BoardView::new(
                            &data.map,
                            &data.pos,
                            &data.player,
                            &data.monsters,
                            &data.stats,
                            &data.potions,
                        );

                        frame.render_widget(
                            GameView::Play(PlayView {
                                board,
                                player_experience,
                                player_stats,
                                level: &data.game_flow.level,
                            }),
                            area,
                        )
                    }
                    GameState::Exit => {}
                };
            })
            .unwrap();
    }
}

pub fn register(dispatcher: &mut DispatcherBuilder, _: &mut World) -> anyhow::Result<()> {
    dispatcher.add_thread_local(RenderSystem);
    Ok(())
}
