use amethyst::{SimpleState, StateData, GameData};
use crate::components::*;

#[derive(Default)]
pub struct GameEnd;

impl SimpleState for GameEnd {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        stage::initialize(data.world);
        player::initialize(data.world);
        block::initialize(data.world);
        score::initialize(data.world);
    }
}