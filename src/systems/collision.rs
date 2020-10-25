use amethyst::{
    core::Transform,
    core::ecs::{Join, System, WriteStorage, ReadStorage, Entities},
};

use crate::components::player::Player;
use crate::components::block::Block;
use crate::components::object::*;
use crate::components::score::Score;
use amethyst::core::ecs::WriteExpect;

pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        WriteExpect<'a, Score>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Block>,
        ReadStorage<'a, Player>,
        Entities<'a>,
    );

    fn run(&mut self, (score, transforms, blocks, players, entities): Self::SystemData) {
        for (player_transform, player) in (&transforms, &players).join() {
            for (block_transform, block, entity) in (&transforms, &blocks, &*entities).join() {
                let is_hit = is_hit(player_transform, &player.size, block_transform, &block.size);
                if is_hit {
                    if block.is_rock {
                        if score.is_dead {
                            return;
                        }
                        score.score += 100;
                    } else {
                        score.is_dead = true;
                    }
                    entities.delete(entity);
                }
            }
        }
    }
}
