use specs::prelude::*;
use specs::{Component, DenseVecStorage, DispatcherBuilder, World, WorldExt};

use crate::board::WorldTileMap;
use crate::combat::{CombatState, CombatStats};
use crate::experience::Experience;
use crate::flow::{GameFlow, GameState};
use crate::movement::{find_free_position, MoveAction, Position};
use crate::term::{Command, TermCommands};

pub mod view;

#[derive(Component)]
pub struct Player {}

struct PlayerSpawnSystem;

impl<'a> specs::System<'a> for PlayerSpawnSystem {
    type SystemData = (
        specs::Entities<'a>,
        specs::WriteStorage<'a, Position>,
        specs::WriteStorage<'a, Player>,
        specs::WriteStorage<'a, CombatStats>,
        specs::WriteStorage<'a, Experience>,
        specs::Read<'a, GameFlow>,
        specs::Read<'a, WorldTileMap>,
    );

    fn run(
        &mut self,
        (entities, mut positions, mut players, mut stats, mut experiences, game_flow, tile_map): Self::SystemData,
    ) {
        let GameState::Started = game_flow.state else {
            return;
        };

        for (e, _) in (&entities, &players).join() {
            // TODO: just change position on the next level
            entities.delete(e).unwrap();
        }

        let player_spawn_position = { find_free_position(&tile_map, positions.join()).unwrap() };

        let player_entity = entities.create();
        players.insert(player_entity, Player {}).unwrap();
        positions
            .insert(player_entity, player_spawn_position)
            .unwrap();
        stats
            .insert(
                player_entity,
                CombatStats {
                    max_hp: 30,
                    hp: 30,
                    defense: 2,
                    power: 5,
                },
            )
            .unwrap();
        experiences
            .insert(
                player_entity,
                Experience {
                    level: 3,
                    exp_count: 74,
                },
            )
            .unwrap();
    }
}

struct PlayerControlSystem;

impl<'a> specs::System<'a> for PlayerControlSystem {
    type SystemData = (
        specs::Entities<'a>,
        specs::WriteStorage<'a, MoveAction>,
        specs::WriteStorage<'a, Player>,
        specs::Read<'a, TermCommands>,
        specs::Read<'a, CombatState>,
        specs::Read<'a, GameFlow>,
    );

    fn run(
        &mut self,
        (entities, mut moves, players, term_events, combat_state, game_flow): Self::SystemData,
    ) {
        let GameState::Running = game_flow.state else {
            return;
        };
        let CombatState::NoCombat = *combat_state else {
            return;
        };

        for event in term_events.0.iter() {
            let (delta_x, delta_y) = match event {
                Command::Up => (0, -1),
                Command::Down => (0, 1),
                Command::Left => (-1, 0),
                Command::Right => (1, 0),
                _ => continue,
            };

            for (_, e) in (&players, &entities).join() {
                moves.insert(e, MoveAction::new(delta_x, delta_y)).unwrap();
            }
        }
    }
}

pub fn register(dispatcher: &mut DispatcherBuilder, world: &mut World) -> anyhow::Result<()> {
    world.register::<Player>();

    dispatcher.add(
        PlayerSpawnSystem,
        "player_spawn_system",
        &["map_generation_system"],
    );
    dispatcher.add(
        PlayerControlSystem,
        "player_move_system",
        &["player_spawn_system"],
    );
    Ok(())
}
