use amethyst::ecs::prelude::*;
use amethyst::core::math::Id;

#[derive(Default,Clone)]
pub struct Score {
    pub score : i32,
    pub time : f32,
    pub is_start : bool,
    pub is_dead: bool,
}

impl Score {
    pub fn new() -> Score {
        Score {
            score : 0,
            time : 0., 
            is_start : false,
            is_dead : false,
        }
    }
}

impl Component for Score {
    type Storage = VecStorage<Self>;
}

pub fn initialize(world: &mut World){
    let mut score_component = Score::new();
    world.insert(score_component);
}