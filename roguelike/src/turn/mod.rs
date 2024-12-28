use specs::{DispatcherBuilder, Join, World};

use crate::{
    combat::Attacked,
    flow::{GameFlow, GameState},
    movement::Moved,
    player::Player,
};

#[derive(PartialEq, Eq, Clone, Default)]
pub enum Turn {
    #[default]
    Player,
    Game,
}

struct TurnSystem;

impl<'a> specs::System<'a> for TurnSystem {
    type SystemData = (
        specs::ReadStorage<'a, Player>,
        specs::ReadStorage<'a, Moved>,
        specs::ReadStorage<'a, Attacked>,
        specs::Write<'a, Turn>,
        specs::Read<'a, GameFlow>,
    );

    fn run(&mut self, (player, moved, attacked, mut turn, game_flow): Self::SystemData) {
        let (GameState::Running | GameState::Started) = game_flow.state else {
            return;
        };

        if game_flow.state == GameState::Started {
            *turn = Turn::Player;
        }

        let player_moved = (&player, &moved).join().next().is_some();
        let player_attacked = (&player, &attacked).join().next().is_some();

        let curr_turn = turn.clone();
        *turn = match curr_turn {
            Turn::Game => Turn::Player,
            Turn::Player if player_moved || player_attacked => Turn::Game,
            Turn::Player => Turn::Player,
        }
    }
}

pub fn register(dispatcher: &mut DispatcherBuilder, world: &mut World) -> anyhow::Result<()> {
    world.insert(Turn::Player);
    dispatcher.add(TurnSystem, "turn_system", &["death_system"]);
    Ok(())
}
