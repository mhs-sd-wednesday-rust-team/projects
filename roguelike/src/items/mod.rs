use anyhow::anyhow;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use rand::seq::IteratorRandom;
use rayon::iter::Inspect;
use specs::prelude::*;
use specs::{Component, DenseVecStorage, DispatcherBuilder, World, WorldExt};

use crate::board::tile::Tile;
use crate::board::WorldTileMap;
use crate::components::{CombatStats, InBackpack, Position, WantsToPickupItem};
use crate::player::Player;
use crate::term::TermEvents;

#[derive(Component)]
pub struct Item {}

#[derive(Component)]
pub struct Potion {
    pub heal_amount: i64,
}

struct ItemCollectionSystem {}

impl<'a> specs::System<'a> for ItemCollectionSystem {
    type SystemData = (
        specs::WriteStorage<'a, Player>,
        specs::WriteStorage<'a, WantsToPickupItem>,
        specs::Write<'a, WorldTileMap>,
        specs::WriteStorage<'a, InBackpack>,
    );

    fn run(&mut self, (player, mut wants_pickup, mut tile_map, mut backpack): Self::SystemData) {
        for pickup in wants_pickup.join() {
            // TODO: Add removing item from map
            backpack
                .insert(
                    pickup.item,
                    InBackpack {
                        owner: pickup.collected_by,
                    },
                )
                .unwrap();
        }
        wants_pickup.clear();
    }
}

pub fn register(dispatcher: &mut DispatcherBuilder, world: &mut World) -> anyhow::Result<()> {
    world.register::<CombatStats>();
    // world.register::<Player>();

    // let player_spawn_position = {
    //     let tile_map = world.read_resource::<WorldTileMap>();
    //     find_player_spawn_position(&tile_map)?
    // };

    // world
    //     .create_entity()
    //     .with(player_spawn_position)
    //     .with(Player {})
    //     .with(CombatStats {
    //         max_hp: 30,
    //         hp: 30,
    //         defense: 2,
    //         power: 5,
    //     })
    //     .build();

    // dispatcher.add(ItemCollectionSystem, "item_collection_system", &[]);
    Ok(())
}
