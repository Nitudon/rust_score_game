use amethyst::{
    core::ecs::{System, Entities, LazyUpdate},
    core::Time,
    shred::{ReadExpect, WriteExpect} 
};
use crate::components::block;
use crate::components::block::*;
use crate::components::score::Score;
use crate::util::rand;
use amethyst::core::ecs::{ReadStorage, Join};
use std::ops::Deref;

const BLOCK_SPAWN_INTERVAL: i32 = 3;

pub struct ScoreSystem {
    pub spawn_interval: i32,
}

impl<'a> System<'a> for ScoreSystem {
    type SystemData = (
        WriteExpect<'a, Score>,
        ReadExpect<'a, AppleResource>,
        ReadExpect<'a, RockResource>,
        ReadExpect<'a, Time>,
        ReadExpect<'a, LazyUpdate>,
        Entities<'a>,
    );

    fn run(&mut self, (mut score, apple_resource, rock_resource, time, lazy_update, entities): Self::SystemData) {
        if !score.is_start {
            return;
        }
        
        score.time += time.delta_seconds();
        let interval = (score.time as i32) / BLOCK_SPAWN_INTERVAL;
        if self.spawn_interval < interval
        {
            self.spawn_interval = interval;
            if rand::create_rand_range() > 0.8 {
                block::create_block(rock_resource.deref(), &lazy_update, &entities);
            } else {
                block::create_block(apple_resource.deref(), &lazy_update, &entities);
            }
        }
    }
}
