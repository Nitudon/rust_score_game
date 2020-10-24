use crate::components::block::*;
use crate::components::score::*;
use amethyst::core::ecs::{Join, Entities, System, WriteStorage, LazyUpdate};
use amethyst::core::{Time, Transform};
use amethyst::shred::ReadExpect;

pub struct BlockSystem;

impl<'a> System<'a> for BlockSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Block>,
        Entities<'a>,
    );

    fn run(&mut self, (mut transforms, mut blocks, entities): Self::SystemData) {
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
