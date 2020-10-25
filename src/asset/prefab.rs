use amethyst::assets::{RonFormat, PrefabLoader};
use amethyst::core::ecs::World;

/*
pub fn load_prefab<T>(prefab_name : &str, world: &mut World) {
    let prefab_handle = world.exec(|loader: PrefabLoader<'_, T>| {
        loader.load(prefab_name, RonFormat, ())
    });
    world.insert(prefab_handle);
}
*/