use amethyst::{
    SimpleState, 
    StateData, 
    GameData,
    StateEvent, 
    Trans, 
    SimpleTrans,
    input::{is_mouse_button_down},
    ui::{Anchor, UiLabelBuilder, UiEvent, UiCreator},
    prelude::*
};
use crate::components::*;
use amethyst::renderer::rendy::wsi::winit::MouseButton;
use amethyst::core::ecs::shred::ResourceId;
use amethyst::core::ecs::Entity;
use crate::state::game::Game;

#[derive(Default)]
pub struct Title {
    title_label : Option<Entity>
}

impl SimpleState for Title {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { mut world, .. } = data;
        
        stage::initialize(world);
        player::initialize(world);
        block::initialize(world);
        score::initialize(world);
        
        world.exec(|mut creator: UiCreator<'_>| {
            self.title_label = Some(creator.create("ui/title/screen.ron", ()));
        });
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        let StateData { mut world, .. } = data;
        if let StateEvent::Window(event) = &event {
            if is_mouse_button_down(&event, MouseButton::Left) {
                let mut entity = self.title_label.unwrap();
                world.delete_entity(entity);
                return Trans::Switch(Box::new(Game::default()))
            }
        }
        Trans::None
    }
}