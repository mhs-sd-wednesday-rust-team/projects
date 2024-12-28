use specs::{Component, DenseVecStorage, DispatcherBuilder, Join, World, WorldExt};

use crate::board::WorldTileMap;
use crate::flow::{GameFlow, GameState};
use crate::movement::{find_free_position, Position};
use crate::player::Player;

pub mod view;

pub const DEFAULT_POTIONS_NUMBER: usize = 10;
#[allow(dead_code)]
pub const DEFAULT_WEAPON_NUMBER: usize = 0;

#[derive(Component)]
pub struct Item {}

#[allow(dead_code)]
#[derive(Component)]
pub struct Potion {
    pub heal_amount: i64,
}

struct PotionSpawnSystem;

impl<'a> specs::System<'a> for PotionSpawnSystem {
    type SystemData = (
        specs::Entities<'a>,
        specs::WriteStorage<'a, Position>,
        specs::WriteStorage<'a, Item>,
        specs::WriteStorage<'a, Potion>,
        specs::Read<'a, GameFlow>,
        specs::Read<'a, WorldTileMap>,
    );

    fn run(
        &mut self,
        (entities, mut positions, mut items, mut potions, game_flow, tile_map): Self::SystemData,
    ) {
        let GameState::Started = game_flow.state else {
            return;
        };

        for (e, _) in (&entities, &items).join() {
            entities.delete(e).unwrap();
        }

        for _ in 0..DEFAULT_POTIONS_NUMBER {
            let potion_pos = { find_free_position(&tile_map, positions.join()).unwrap() };

            let potion_entity = entities.create();
            positions.insert(potion_entity, potion_pos).unwrap();
            items.insert(potion_entity, Item {}).unwrap();
            potions
                .insert(potion_entity, Potion { heal_amount: 5 })
                .unwrap();
        }
    }
}

struct ItemCollectionSystem;

impl<'a> specs::System<'a> for ItemCollectionSystem {
    type SystemData = (
        specs::WriteStorage<'a, Player>,
        // specs::WriteStorage<'a, WantsToPickupItem>,
        specs::Write<'a, WorldTileMap>,
        // specs::WriteStorage<'a, InBackpack>,
    );

    fn run(&mut self, _data: Self::SystemData) {
        // for pickup in wants_pickup.join() {
        //     // TODO: Add removing item from map
        //     backpack
        //         .insert(
        //             pickup.item,
        //             InBackpack {
        //                 owner: pickup.collected_by,
        //             },
        //         )
        //         .unwrap();
        // }
        // wants_pickup.clear();
    }
}

pub fn register(dispatcher: &mut DispatcherBuilder, world: &mut World) -> anyhow::Result<()> {
    world.register::<Potion>();
    world.register::<Item>();

    dispatcher.add(
        PotionSpawnSystem,
        "potion_spawn_system",
        &["map_generation_system"],
    );
    dispatcher.add(
        ItemCollectionSystem,
        "item_collection_system",
        &["potion_spawn_system"],
    );
    Ok(())
}
