use amethyst::{SimpleState, StateData, GameData, core::ecs::WorldExt, ui::UiCreator, SimpleTrans, Trans};
use crate::components::*;
use crate::components::score::Score;
use amethyst::ui::{UiText, UiFinder};
use amethyst::core::ecs::Entity;

#[derive(Default)]
pub struct Game {
    pub score_text : Option<Entity>
} 

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { mut world, .. } = data;

        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/game/screen.ron", ());
        });

        let mut score = world.fetch_mut::<Score>();
        score.is_start = true;
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = state_data;

        if self.score_text.is_none() {
            world.exec(|finder: UiFinder<'_>| {
                if let Some(entity) = finder.find("score_text") {
                    self.score_text = Some(entity);
                }
            });
        }
        
        let score = world.fetch::<Score>().score;
        let mut ui_text = world.write_storage::<UiText>();
        {
            if let Some(score_text) = self.score_text.and_then(|entity| ui_text.get_mut(entity)) {
                score_text.text = score.to_string(); 
            }
        }
        
        Trans::None
    }
}