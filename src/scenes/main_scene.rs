use crate::scenes::pickable_unit::PickableUnit;
use gdnative::{
  api::{GlobalConstants, InputEvent, InputEventMouseButton},
  core_types::VariantArray,
  Ref, TRef,
};
use godot::methods;
use godot::prelude::{FromVariant, NativeClass, Node2D, ToVariant};

#[derive(ToVariant, FromVariant, NativeClass)]
#[inherit(Node2D)]
pub struct MainScene {
  held_object: Option<PickableUnit>,
}

#[methods]
impl MainScene {
  fn _init(_owner: Node2D) -> Self {
    Self { held_object: None }
  }

  #[export]
  fn _ready(&self, owner: TRef<Node2D>) {
    for n in 0..owner.get_child_count() {
      let node = unsafe { owner.get_child(n.into()).unwrap().assume_safe() };

      if !node.is_in_group("pickable") {
        continue;
      };

      node
        .connect(
          "clicked",
          owner,
          "_on_pickable_clicked",
          VariantArray::new_shared(),
          0,
        )
        .ok();
    }
  }

  #[export]
  fn _on_pickable_clicked(
    &mut self,
    owner: &Node2D,
    object: Option<PickableUnit>,
  ) {
    match self.held_object {
      None => {
        match object {
          Some(mut o) => {
            o.pickup(owner);
            self.held_object = Some(o);
          }
          _ => {}
        };
      }
      _ => {}
    }
  }

  #[export]
  fn _unhandled_input(&mut self, owner: &Node2D, event: Ref<InputEvent>) {
    let ev = unsafe { event.assume_safe() };

    if let Some(e) = ev.cast::<InputEventMouseButton>() {
      if e.button_index() == GlobalConstants::BUTTON_LEFT && !e.is_pressed() {
        if let Some(obj) = &mut self.held_object {
          obj.drop(owner);
          self.held_object = None;
        }
      }
    }
  }
}

impl MainScene {
  fn new(_owner: &Node2D) -> Self {
    MainScene { held_object: None }
  }
}
