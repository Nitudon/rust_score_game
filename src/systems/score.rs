use amethyst::{
    core::ecs::{System, Entities, LazyUpdate},
    core::Time,
    shred::{ReadExpect, WriteExpect} 
};
use crate::components::block;
use crate::components::block::BlockResource;
use crate::components::score::Score;
use crate::util::rand;
use amethyst::core::ecs::{ReadStorage, Join};

const BLOCK_SPAWN_INTERVAL: i32 = 3;

pub struct ScoreSystem {
    pub spawn_interval: i32,
}

impl<'a> System<'a> for ScoreSystem {
    type SystemData = (
        WriteExpect<'a, Score>,
        ReadStorage<'a, BlockResource>,
        ReadExpect<'a, Time>,
        ReadExpect<'a, LazyUpdate>,
        Entities<'a>,
    );

    fn run(&mut self, (mut score, block_resources, time, lazy_update, entities): Self::SystemData) {
        score.time += time.delta_seconds();
        let interval = (score.time as i32) / BLOCK_SPAWN_INTERVAL;
        if self.spawn_interval < interval
        {
            self.spawn_interval = interval;
            let is_rock = rand::create_rand_range() > 0.8;
            println!("{}", (&block_resources).join().count());
            for block_resource in (&block_resources).join() {
                let is_rock = rand::create_rand_range() > 0.8;
                if is_rock {
                    if block_resource.block.is_rock {
                        block::create_block(block_resource, &lazy_update, &entities); 
                    }
                } else {
                    if !block_resource.block.is_rock {
                        block::create_block(block_resource, &lazy_update, &entities);
                    }
                }
            }
        }
    }
}
