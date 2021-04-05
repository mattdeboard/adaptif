use gdnative::{
  api::{InputEvent, InputEventMouseButton, Viewport},
  core_types::VariantArray,
  Ref,
};
use godot::api::GlobalConstants;
use godot::methods;
use godot::prelude::{FromVariant, NativeClass, Node2D};

#[derive(Debug, ToVariant, FromVariant)]
enum RigidBody2DMode {
  Rigid = 0,
  Static = 1,
  // Character = 2,
  // Kinematic = 3,
}

#[derive(Debug, FromVariant, ToVariant, NativeClass)]
#[inherit(Node2D)]
pub struct PickableUnit {
  held: bool,
  mode: RigidBody2DMode,
}

#[methods]
impl PickableUnit {
  fn _init(_owner: Node2D) -> Self {
    PickableUnit {
      held: false,
      mode: RigidBody2DMode::Rigid,
    }
  }

  #[export]
  fn _ready(&self, _owner: &Node2D) {
    _owner.add_user_signal("clicked", VariantArray::new_shared());
    godot_warn!("_ready called")
  }

  #[export]
  fn _input_event(
    &self,
    owner: &Node2D,
    _viewport: Ref<Viewport>,
    event: Ref<InputEvent>,
    _shape_idx: u32,
  ) {
    let ev = unsafe { event.assume_safe() };
    if let Some(e) = ev.cast::<InputEventMouseButton>() {
      if e.button_index() == GlobalConstants::BUTTON_LEFT && e.is_pressed() {
        owner.emit_signal("clicked", &[]);
      }
    }
  }

  #[export]
  pub fn pickup(&mut self, _owner: &Node2D) {
    if self.held {
      return;
    }

    self.mode = RigidBody2DMode::Static;
    self.held = true;
  }

  #[export]
  pub fn drop(&mut self, _owner: &Node2D) {
    if self.held {
      self.mode = RigidBody2DMode::Rigid;
      self.held = false;
    }
  }
}

impl PickableUnit {
  fn new(_owner: &Node2D) -> Self {
    PickableUnit {
      held: false,
      mode: RigidBody2DMode::Rigid,
    }
  }

  // fn _init(_owner: Node2D) -> Self {
  //   PickableUnit {
  //     held: false,
  //     mode: RigidBody2DMode::Rigid,
  //   }
  // }
}
