#[macro_use]
extern crate gdnative;

mod build;
mod game_status;
mod scenes;

fn init(handle: gdnative::nativescript::InitHandle) {
  handle.add_class::<scenes::main_scene::MainScene>();
  handle.add_class::<scenes::pickable_unit::PickableUnit>();
}

godot_init!(init);
