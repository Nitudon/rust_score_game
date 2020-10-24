use amethyst::ecs::prelude::*;

#[derive(Clone)]
pub struct Score {
    pub score : i32,
    pub time : f32,
}

impl Score {
    pub fn new() -> Score {
        Score {
            score : 0,
            time : 0., 
        }
    }
}

impl Component for Score {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialize(world: &mut World) {
    let mut score_component = Score::new();
    world.insert(score_component.clone());
}