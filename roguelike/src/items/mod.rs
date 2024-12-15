use anyhow::anyhow;
use rand::seq::IteratorRandom;
use specs::{Builder, Component, DenseVecStorage, DispatcherBuilder, World, WorldExt};

use crate::board::tile::Tile;
use crate::board::WorldTileMap;
use crate::components::Position;
use crate::player::Player;

pub mod view;

pub const DEFAULT_POTIONS_NUMBER: usize = 10;
pub const DEFAULT_WEAPON_NUMBER: usize = 0;

#[derive(Component)]
pub struct Item {}

#[derive(Component)]
pub struct Potion {
    pub heal_amount: i64,
}

struct ItemCollectionSystem;

// impl<'a> specs::System<'a> for ItemCollectionSystem {
//     type SystemData = (
//         specs::WriteStorage<'a, Player>,
//         // specs::WriteStorage<'a, WantsToPickupItem>,
//         specs::Write<'a, WorldTileMap>,
//         // specs::WriteStorage<'a, InBackpack>,
//     );

//     fn run(&mut self, _data: Self::SystemData) {
//         // for pickup in wants_pickup.join() {
//         //     // TODO: Add removing item from map
//         //     backpack
//         //         .insert(
//         //             pickup.item,
//         //             InBackpack {
//         //                 owner: pickup.collected_by,
//         //             },
//         //         )
//         //         .unwrap();
//         // }
//         // wants_pickup.clear();
//     }
// }

pub fn find_potion_position(
    map: &WorldTileMap,
    item_positions: &Vec<Position>,
) -> anyhow::Result<Position> {
    let mut rng = rand::thread_rng();

    let spawn_position = (0..map.height)
        .zip(0..map.width)
        .filter(|&pos| {
            matches!(map.board[pos.0][pos.1], Tile::Ground)
                && !item_positions.contains(&Position {
                    x: pos.1 as i64,
                    y: pos.0 as i64,
                })
        })
        .choose(&mut rng)
        .ok_or(anyhow!("Did not find any ground tile to spawn creature"))?;

    let pos = Position {
        x: spawn_position.1 as i64,
        y: spawn_position.0 as i64,
    };
    Ok(pos)
}

pub fn register(dispatcher: &mut DispatcherBuilder, world: &mut World) -> anyhow::Result<()> {
    world.register::<Potion>();

    let mut item_spawn_positions =
        Vec::with_capacity(DEFAULT_POTIONS_NUMBER + DEFAULT_WEAPON_NUMBER);

    for _ in 0..DEFAULT_POTIONS_NUMBER {
        let potion_pos = {
            let tile_map = world.read_resource::<WorldTileMap>();
            find_potion_position(&tile_map, &item_spawn_positions)?
        };
        item_spawn_positions.push(potion_pos);
        world
            .create_entity()
            .with(potion_pos)
            .with(Potion { heal_amount: 5 })
            .build();
    }

    // dispatcher.add(ItemCollectionSystem, "item_collection_system", &[]);
    Ok(())
}
