use specs::{DispatcherBuilder, Join, World};

use crate::{movement::Moved, player::Player};

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
        specs::Write<'a, Turn>,
    );

    fn run(&mut self, (player, moved, mut turn): Self::SystemData) {
        let player_moved = (&player, &moved).join().next().is_some();

        let curr_turn = turn.clone();
        *turn = match curr_turn {
            Turn::Game => Turn::Player,
            Turn::Player if player_moved => Turn::Game,
            Turn::Player => Turn::Player,
        }
    }
}

pub fn register(dispatcher: &mut DispatcherBuilder, world: &mut World) -> anyhow::Result<()> {
    world.insert(Turn::Player);
    dispatcher.add(TurnSystem, "turn_system", &["death_system"]);
    Ok(())
}
