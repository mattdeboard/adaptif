use crate::scenes::pickable_unit::PickableUnit;
use gdnative::{
  api::{GlobalConstants, InputEvent, InputEventMouseButton, Object},
  core_types::{GodotString, VariantArray},
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
  fn _ready(&self, _owner: TRef<Node2D>) {
    if let Some(tree) = _owner.get_tree() {
      let ref _tree = unsafe { tree.assume_safe() };
      let nodes = _tree.get_nodes_in_group("pickable");

      for n in 0..nodes.len() {
        let node: Option<Ref<Object>> = nodes.get(n).try_to_object();

        if let Some(_node) = node {
          let safe_node = unsafe { _node.assume_safe() };
          let variant_array = VariantArray::new_shared();

          unsafe {
            variant_array.push(_owner);
            variant_array.push(safe_node.cast::<Node2D>().unwrap().to_variant())
          }
          safe_node
            .connect(
              GodotString::from_str("clicked"),
              _owner,
              GodotString::from_str("_on_pickable_clicked"),
              variant_array,
              0,
            )
            .ok();
        }
      }
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
