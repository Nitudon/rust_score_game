use amethyst::{SimpleState, StateData, GameData, ui::{Anchor, UiLabelBuilder, UiEvent}, StateEvent, Trans, SimpleTrans};
use crate::components::*;

#[derive(Default)]
pub struct Title;

impl SimpleState for Title {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        stage::initialize(data.world);
        
        UiLabelBuilder::<String>::new("Game Start".to_string())
            .with_font_size(32.0)
            .with_position(0., 120.)
            .with_anchor(Anchor::BottomMiddle)
            .build_from_world(&data.world);
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