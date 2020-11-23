use amethyst::{
    core::ecs::{Join, Entities, System, WriteStorage, LazyUpdate}, 
    core::{Time, Transform}, 
    shred::ReadExpect,
};

use crate::components::block::*;
use crate::components::score::*;

pub struct BlockSystem;

impl<'a> System<'a> for BlockSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Block>,
        ReadExpect<'a, Score>,
        Entities<'a>,
    );

    fn run(&mut self, (mut transforms, mut blocks, score, entities): Self::SystemData) {
        if !score.is_start || score.is_dead  {
            return;
        }
        
        for (transform, block, entity) in (&mut transforms, &mut blocks, &* entities).join()  {
            let y = transform.translation().y;
            let velocity = block.velocity.y;
            let position_y = y - velocity;

            transform.set_translation_y(position_y);
            if position_y <= 0.
            {
                entities.delete(entity);
            }
        }
    }
}
