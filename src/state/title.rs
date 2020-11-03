use amethyst::{
    SimpleState, 
    StateData, 
    GameData,
    StateEvent, 
    Trans, 
    SimpleTrans,
    ui::{Anchor, UiLabelBuilder, UiEvent, UiCreator},
};
use crate::components::*;

#[derive(Default)]
pub struct Title;

impl SimpleState for Title {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { mut world, .. } = data;
        
        stage::initialize(world);
        player::initialize(world);
        block::initialize(world);
        score::initialize(world);
        
        world.exec(|mut creator: UiCreator<'_>| {
            
            creator.create("ui/title/screen.ron", ());
        });
    }

    fn handle_event(&mut self, _: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        match &event {
            StateEvent::Ui(ui_event) => {
                Trans::None
            }
            StateEvent::Input(input) => {
                Trans::None
            }
            _ => Trans::None
        }
    }
}