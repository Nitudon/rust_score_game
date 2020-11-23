use amethyst::{SimpleState, StateData, GameData};
use crate::components::*;
use crate::components::score::Score;
use amethyst::ui::UiCreator;

#[derive(Default)]
pub struct GameEnd;

impl SimpleState for GameEnd {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { mut world, .. } = data;
        
        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/game_end/screen.ron", ());
        });
    }
}