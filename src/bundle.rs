use amethyst::{
    Result,
    core::bundle::{SystemBundle},
    ecs::DispatcherBuilder,
    prelude::World
};
use crate::systems::*;
use crate::systems::block::BlockSystem;
use crate::systems::score::ScoreSystem;

pub struct GameBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameBundle {
    fn build(self, _world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        let score_system = ScoreSystem::new();

        builder.add(player::PlayerSystem, "player_system", &[]);
        builder.add(block::BlockSystem, "block_system", &[]);
        builder.add(collision::CollisionSystem, "collision_system", &[]);
        builder.add(score_system, "score_system", &[]);
        Ok(())
    }
}