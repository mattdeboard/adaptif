use gdnative::api::GlobalConstants;
use gdnative::methods;
use gdnative::prelude::{NativeClass, Reference};
use gdnative::{
  api::{InputEvent, InputEventMouseButton, Viewport},
  core_types::OwnedToVariant,
  prelude::{ClassBuilder, Signal},
  Ref, TRef,
};

#[derive(Copy, Clone, Debug)]
enum RigidBody2DMode {
  Rigid = 0,
  Static = 1,
  // Character = 2,
  // Kinematic = 3,
}

#[derive(Copy, Clone, NativeClass)]
#[inherit(Reference)]
#[register_with(Self::register_signals)]
// #[no_constructor]
pub struct PickableUnit {
  held: bool,
  mode: RigidBody2DMode,
}

#[methods]
impl PickableUnit {
  fn new(_owner: &Reference) -> Self {
    PickableUnit {
      held: false,
      mode: RigidBody2DMode::Rigid,
    }
  }

  #[export]
  fn _input_event(
    &self,
    owner: &Reference,
    _viewport: Ref<Viewport>,
    event: Ref<InputEvent>,
    _shape_idx: u32,
  ) {
    let ev = unsafe { event.assume_safe() };
    if let Some(e) = ev.cast::<InputEventMouseButton>() {
      if e.button_index() == GlobalConstants::BUTTON_LEFT && e.is_pressed() {
        let instance = self.emplace();
        owner.emit_signal("clicked", &[instance.owned_to_variant()]);
      }
    }
  }

  #[export]
  fn _physics_process(&self, owner: &Reference, _delta: f64) {
    if self.held {
      // owner.set_position(owner.get_global_mouse_position());
    }
  }

  #[export]
  pub fn pickup(&mut self, _owner: TRef<Reference>) {
    if !self.held {
      self.mode = RigidBody2DMode::Static;
      self.held = true;
    }
  }

  #[export]
  pub fn drop(&mut self, _owner: TRef<Reference>) {
    if self.held {
      self.mode = RigidBody2DMode::Rigid;
      self.held = false;
    }
  }

  fn register_signals(builder: &ClassBuilder<Self>) {
    builder.add_signal(Signal {
      name: "clicked",
      args: &[],
    });
  }
}
