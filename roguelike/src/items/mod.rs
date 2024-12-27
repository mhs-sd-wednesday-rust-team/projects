use anyhow::anyhow;
use rand::seq::IteratorRandom;
use specs::{Component, DenseVecStorage, DispatcherBuilder, World, WorldExt};

use crate::board::tile::Tile;
use crate::board::WorldTileMap;
use crate::movement::Position;
use crate::player::Player;

pub mod view;

pub const DEFAULT_POTIONS_NUMBER: usize = 10;
pub const DEFAULT_WEAPON_NUMBER: usize = 0;

#[derive(Component)]
pub struct Item {}

#[allow(dead_code)]
#[derive(Component)]
pub struct Potion {
    pub heal_amount: i64,
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

pub fn find_item_spawn_position(
    map: &WorldTileMap,
    item_positions: &mut [Position],
) -> anyhow::Result<Position> {
    let mut rng = rand::thread_rng();

    let spawn_position = (0..map.height)
        .flat_map(|y| (0..map.width).map(move |x| (y, x)))
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

    dispatcher.add(ItemCollectionSystem, "item_collection_system", &[]);
    Ok(())
}
