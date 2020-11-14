use amethyst::{
    SimpleState, 
    StateData, 
    GameData,
    
};
use crate::components::*;
use crate::components::score::Score;
use amethyst::core::ecs::WorldExt;

#[derive(Default)]
pub struct Game; 

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { mut world, .. } = data;
        world.fetch_mut::<Score>().is_start = true;
    }
}