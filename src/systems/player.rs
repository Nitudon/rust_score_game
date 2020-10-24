use amethyst::{
    core::ecs::{System, WriteStorage, Read},
    input::{InputHandler, StringBindings},
    core::Transform,
};
use crate::components::player;
use amethyst::core::ecs::Join;
use amethyst::input::VirtualKeyCode;
use crate::components::object::Object;
use amethyst::core::num;
use amethyst::shred::ReadExpect;

pub struct PlayerSystem;

pub const PLAYER_POSITION_X_MIN: f32 = 40.0;
pub const PLAYER_POSITION_X_MAX: f32 = 920.0;

impl<'a> System<'a> for PlayerSystem {
    type SystemData = (
        WriteStorage<'a, Transform>, 
        WriteStorage<'a, player::Player>,
        ReadExpect<'a, InputHandler<StringBindings>>, 
    );

    fn run(&mut self, (mut transforms, mut players, input): Self::SystemData) {
        for (transform, player) in (&mut transforms, &mut players).join() {
            if input.key_is_down(VirtualKeyCode::Right) {
                player.set_velocity(6., 0.);
            }
            else if input.key_is_down(VirtualKeyCode::Left) {
                player.set_velocity(-6., 0.);
            }
            else { 
                player.set_velocity(0., 0.);
            }

            let player_position = num::clamp(transform.translation().x + player.get_velocity().x, PLAYER_POSITION_X_MIN, PLAYER_POSITION_X_MAX);
            transform.set_translation_x(player_position);
        }
    }
}
