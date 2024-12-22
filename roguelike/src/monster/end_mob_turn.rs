use specs::prelude::*;

use crate::flow::{GameFlow, GameState, RunningState};

struct EndMobTurnSystem;

impl<'a> specs::System<'a> for EndMobTurnSystem {
    type SystemData = (specs::Write<'a, GameFlow>,);

    fn run(&mut self, (mut game_flow,): Self::SystemData) {
        if game_flow.state != GameState::Finished {
            game_flow.state = GameState::Running(RunningState::PlayerTurn)
        }
    }
}

pub fn register_systems(dispatcher: &mut DispatcherBuilder) -> Result<(), String> {
    dispatcher.add(
        EndMobTurnSystem,
        "end_mob_turn",
        &["monster_system", "split_monster_ability"],
    );
    Ok(())
}
