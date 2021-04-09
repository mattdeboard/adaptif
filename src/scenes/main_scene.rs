use crate::scenes::pickable_unit::PickableUnit;
use gdnative::{
  api::{GlobalConstants, InputEvent, InputEventMouseButton, Reference},
  core_types::{Variant, VariantArray},
  Ref, TRef,
};
use godot::methods;
use godot::prelude::{Instance, NativeClass, Node2D, Shared};

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct MainScene {
  #[property]
  held_object: Option<Instance<PickableUnit, Shared>>,
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
  fn _on_pickable_clicked(&mut self, owner: &Node2D, object: Variant) {
    let base_obj: Ref<Reference> =
      object.try_to_object().expect("is Reference");
    let instance: Instance<PickableUnit, Shared> =
      Instance::from_base(base_obj).unwrap();
    instance.map(|unit: &PickableUnit, _base| {
      let my_ref = unsafe { base_obj.assume_safe() };
      unit.pickup(my_ref);
      self.held_object = Some(instance);
    });
  }

  #[export]
  fn _unhandled_input(&mut self, owner: &Node2D, event: Ref<InputEvent>) {
    let ev = unsafe { event.assume_safe() };

    if let Some(e) = ev.cast::<InputEventMouseButton>() {
      if e.button_index() == GlobalConstants::BUTTON_LEFT && !e.is_pressed() {
        if let Some(obj) = &mut self.held_object {
          let base_obj = obj.into_base();
          let instance: Instance<PickableUnit, Shared> =
            Instance::from_base(base_obj).unwrap();
          instance.map(|unit: &PickableUnit, _base| {
            let my_ref = unsafe { base_obj.assume_safe() };
            unit.drop(my_ref);
            self.held_object = None;
          });
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
