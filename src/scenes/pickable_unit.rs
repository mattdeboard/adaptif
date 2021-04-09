use gdnative::{
  api::{InputEvent, InputEventMouseButton, Viewport},
  prelude::{ClassBuilder, Signal},
  Ref,
};
use godot::api::GlobalConstants;
use godot::methods;
use godot::prelude::{FromVariant, NativeClass, Node2D, ToVariant};

#[derive(Debug, ToVariant, FromVariant)]
enum RigidBody2DMode {
  Rigid = 0,
  Static = 1,
  // Character = 2,
  // Kinematic = 3,
}

#[derive(Debug, FromVariant, ToVariant, NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::register_signals)]
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
        owner.emit_signal("clicked", &[self.to_variant()]);
      }
    }
  }

  #[export]
  fn _physics_process(&self, owner: &Node2D, _delta: f64) {
    if self.held {
      owner.set_global_position(owner.get_global_mouse_position());
    }
  }

  #[export]
  pub fn pickup(&mut self, _owner: &Node2D) {
    if !self.held {
      self.mode = RigidBody2DMode::Static;
      self.held = true;
    }
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

  fn register_signals(builder: &ClassBuilder<Self>) {
    builder.add_signal(Signal {
      name: "clicked",
      args: &[],
    });
  }
}
