use std::collections::HashSet;

use rand::{rngs::ThreadRng, seq::IteratorRandom, Rng};
use specs::{prelude::*, Component};

use crate::{
    board::{tile::Tile, WorldTileMap},
    combat::{CombatState, CombatStats},
    experience::KillExperience,
    flow::{GameFlow, GameState},
    movement::Position,
    turn::Turn,
};

use super::Monster;

#[derive(Component, Clone, Copy)]
pub struct SplitMonsterAbility {
    split_probability_pct: i64,
}

impl SplitMonsterAbility {
    pub fn new(split_probability_pct: i64) -> Self {
        if !(0..=100).contains(&split_probability_pct) {
            panic!("split_probability_pct must be between 0 and 100");
        }
        Self {
            split_probability_pct,
        }
    }
}

struct SplitMonsterAbilitySystem;

impl SplitMonsterAbilitySystem {
    // Если свободных клеток не хватит, то вернем сколько сможем
    fn get_spawn_position(
        &self,
        rng: &mut ThreadRng,
        occupied_positions: &HashSet<Position>,
        map: &WorldTileMap,
        target_position: &Position,
    ) -> Option<Position> {
        (-1..=1)
            .flat_map(|dy| (-1..=1).map(move |dx| (dy, dx)))
            .filter(|&(dy, dx)| (dy != 0 || dx != 0))
            .map(|(dy, dx)| Position {
                x: target_position.x + dx,
                y: target_position.y + dy,
            })
            .filter(|pos| {
                (0 <= pos.x && pos.x < map.width as i64)
                    && (0 <= pos.y && pos.y < map.height as i64)
                    && matches!(map.board[pos.y as usize][pos.x as usize], Tile::Ground)
                    && !occupied_positions.contains(pos)
            })
            .choose(rng)
    }
}

impl<'a> specs::System<'a> for SplitMonsterAbilitySystem {
    type SystemData = (
        specs::Read<'a, WorldTileMap>,
        specs::Entities<'a>,
        specs::ReadStorage<'a, Position>,
        specs::ReadStorage<'a, Monster>,
        specs::ReadStorage<'a, CombatStats>,
        specs::ReadStorage<'a, SplitMonsterAbility>,
        specs::ReadStorage<'a, KillExperience>,
        specs::Read<'a, LazyUpdate>,
        specs::Read<'a, GameFlow>,
        specs::Read<'a, CombatState>,
        specs::Read<'a, Turn>,
    );

    fn run(
        &mut self,
        (
            world_tile_map,
            entities,
            positions,
            monsters,
            stats,
            split_abilities,
            kill_experiences,
            updater,
            game_flow,
            combat_state,
            turn,
        ): Self::SystemData,
    ) {
        let GameState::Running = game_flow.state else {
            return;
        };
        let CombatState::NoCombat = *combat_state else {
            return;
        };

        let Turn::Game = *turn else {
            return;
        };

        let mut rng = rand::thread_rng();

        let need_split: Vec<_> = (
            &monsters,
            &split_abilities,
            &positions,
            &stats,
            &kill_experiences,
        )
            .join()
            .filter(|(_, split_ability, _pos, _stats, _kill_exp)| {
                rng.gen_range(0..100) < split_ability.split_probability_pct
            })
            .collect();

        if need_split.is_empty() {
            return;
        }

        let positions_cloned = positions.as_slice().iter().cloned();
        let mut occupied_positions = HashSet::<Position>::from_iter(positions_cloned);

        for (monster, split_ability, old_pos, stat, kill_exp) in need_split.into_iter() {
            if stat.max_hp <= 2 {
                continue;
            }

            let Some(new_pos) =
                self.get_spawn_position(&mut rng, &occupied_positions, &world_tile_map, old_pos)
            else {
                continue;
            };

            let new_monster_entity = entities.create();
            updater.insert(new_monster_entity, *monster);
            updater.insert(new_monster_entity, *split_ability);
            updater.insert(new_monster_entity, new_pos);

            let mut new_stat = (*stat).clone();
            new_stat.hp -= 2;
            new_stat.max_hp -= 2;
            updater.insert(new_monster_entity, new_stat);

            let mut new_kill_exp = (*kill_exp).clone();
            new_kill_exp.exp_count = new_kill_exp.exp_count.saturating_sub(2);
            updater.insert(new_monster_entity, new_kill_exp);

            occupied_positions.insert(new_pos);
        }
    }
}

pub fn register(dispatcher: &mut DispatcherBuilder, world: &mut World) -> anyhow::Result<()> {
    world.register::<SplitMonsterAbility>();
    dispatcher.add(
        SplitMonsterAbilitySystem,
        "monster_split_system",
        &["monster_spawn_system"],
    );
    Ok(())
}