use specs::{prelude::ResourceId, DispatcherBuilder, Join, SystemData, World};

use crate::{
    board::WorldTileMap,
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
                    GameState::Running(_) => {
                        let (_, player_pos, player_exp) =
                            (&data.player, &data.pos, &data.experience)
                                .join()
                                .next()
                                .expect("should be a player");

                        let monsters: Vec<&Position> = (&data.pos, &data.monsters)
                            .join()
                            .map(|(pos, _)| pos)
                            .collect();
                        let potions: Vec<&Position> = (&data.pos, &data.potions)
                            .join()
                            .map(|(pos, _)| pos)
                            .collect();

                        frame.render_widget(
                            GameView::Play(PlayView {
                                map: &data.map,
                                player: player_pos,
                                player_experience: player_exp,
                                monsters,
                                potions,
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
