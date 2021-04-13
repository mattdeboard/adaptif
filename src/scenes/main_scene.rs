use crate::scenes::pickable_unit::PickableUnit;
use gdnative::methods;
use gdnative::prelude::{Instance, NativeClass, Node2D, Shared};
use gdnative::{
  api::Reference,
  core_types::{Variant, VariantArray},
  Ref, TRef,
};

#[derive(NativeClass)]
#[inherit(Node2D)]
#[no_constructor]
pub struct MainScene {
  held_object: Option<Ref<Reference>>,
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

  // #[export]
  // fn _on_pickable_clicked(&mut self, owner: &Node2D, object: Variant) {
  //   let base_obj: Ref<Reference> =
  //     object.try_to_object().expect("is Reference");
  //   let instance: Instance<PickableUnit, Shared> =
  //     Instance::from_base(base_obj).unwrap();
  //   unsafe {
  //     instance.assume_safe().map(|unit: &PickableUnit, _base| {
  //       let my_ref = unsafe { base_obj.assume_safe() };
  //       unit.pickup(base_obj);
  //       self.held_object = Some(base_obj);
  //     });
  //   }
  // }

  // #[export]
  // fn _unhandled_input(&mut self, owner: &Node2D, event: Ref<InputEvent>) {
  //   let ev = unsafe { event.assume_safe() };

  //   if let Some(e) = ev.cast::<InputEventMouseButton>() {
  //     if e.button_index() == GlobalConstants::BUTTON_LEFT && !e.is_pressed() {
  //       if let Some(ref obj) = &self.held_object {
  //         let base_obj = obj.into_base();
  //         let instance: Instance<PickableUnit, Shared> =
  //           Instance::from_base(base_obj).unwrap();
  //         unsafe {
  //           instance.assume_unique().map(|unit: &PickableUnit, _base| {
  //             // let my_ref = unsafe { base_obj.assume_safe() };
  //             unit.drop(base_obj.assume_safe());
  //             self.held_object = None;
  //           });
  //         }
  //       }
  //     }
  //   }
  // }
}

// impl MainScene {
//   fn new(_owner: &Node2D) -> Self {
//     MainScene { held_object: None }
//   }
// }
