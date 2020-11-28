use amethyst::{
    core::ecs::{System, Entities, LazyUpdate},
    core::Time,
    shred::{ReadExpect, WriteExpect} 
};
use crate::components::block;
use crate::components::block::*;
use crate::components::score::Score;
use crate::util::rand;
use amethyst::core::ecs::{ReadStorage, Join, WriteStorage};
use std::ops::Deref;
use amethyst::ui::UiText;
use std::cmp::max;

const DEFAULT_SPAWN_INTERVAL: f32 = 2.0;
const MIN_SPAWN_INTERVAL: f32 = 0.15;
const CHANGE_SPAWN_INTERVAL: f32 = 0.05;

pub struct ScoreSystem {
    pub spawn_interval: f32,
    pub spawn_count: i32,
}

impl ScoreSystem {
    pub fn new() -> ScoreSystem {
        ScoreSystem {
            spawn_interval: DEFAULT_SPAWN_INTERVAL,
            spawn_count: 0
        }
    }
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
        if !score.is_start || score.is_dead {
            return;
        }
        
        score.time += time.delta_seconds();
        if self.spawn_interval < score.time
        {
            let mut interval = DEFAULT_SPAWN_INTERVAL - CHANGE_SPAWN_INTERVAL * (self.spawn_count as f32);
            if interval < MIN_SPAWN_INTERVAL {
                interval = MIN_SPAWN_INTERVAL;
            }
            self.spawn_interval += interval;
            self.spawn_count += 1;
            if rand::create_rand_range() > 0.8 {
                block::create_block(rock_resource.deref(), &lazy_update, &entities);
            } else {
                block::create_block(apple_resource.deref(), &lazy_update, &entities);
            }
        }
    }
}
