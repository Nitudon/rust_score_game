use amethyst::input::{InputBundle, StringBindings};

pub fn create_input_bundle() -> InputBundle<StringBindings> {
    InputBundle::<StringBindings>::new()
}