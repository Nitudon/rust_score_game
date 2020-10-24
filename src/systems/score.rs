use crate::components::score::{Score};
use amethyst::core::ecs::{System, Entities, LazyUpdate};
use amethyst::core::Time;
use amethyst::shred::{ReadExpect, WriteExpect};
use crate::components::block;
use crate::components::block::BlockResource;

const BLOCK_SPAWN_INTERVAL: i32 = 3;

pub struct ScoreSystem {
    pub spawn_interval: i32,
}

impl<'a> System<'a> for ScoreSystem {
    type SystemData = (
        WriteExpect<'a, Score>,
        ReadExpect<'a, Time>,
        ReadExpect<'a, BlockResource>,
        ReadExpect<'a, LazyUpdate>,
        Entities<'a>,
    );

    fn run(&mut self, (mut score, time, block_resource, lazy_update, entities): Self::SystemData) {
        score.time += time.delta_seconds();
        let interval = (score.time as i32) / BLOCK_SPAWN_INTERVAL;
        if self.spawn_interval < interval
        {
            self.spawn_interval = interval;
            block::create_block(&block_resource, &lazy_update, &entities);
        }
    }
}
