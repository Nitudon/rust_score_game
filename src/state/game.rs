use amethyst::{SimpleState, StateData, GameData};
use crate::components::*;

#[derive(Default)]
pub struct Game;

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        player::initialize(data.world);
        block::initialize(data.world);
        score::initialize(data.world);
    }
}