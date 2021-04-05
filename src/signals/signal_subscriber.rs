use gdnative::prelude::*;

#[derive(ToVariant, FromVariant, NativeClass)]
#[inherit(Label)]
pub struct SignalSubscriber {
  times_received: i32,
}

#[methods]
impl SignalSubscriber {
  fn new(_owner: &Label) -> Self {
    SignalSubscriber { times_received: 0 }
  }

  #[export]
  fn _ready(&mut self, owner: TRef<Label>) {
    let emitter = &mut owner.get_node("../SignalEmitter").unwrap();
    let emitter = unsafe { emitter.assume_safe() };

    emitter
      .connect("tick", owner, "notify", VariantArray::new_shared(), 0)
      .unwrap();
    emitter
      .connect(
        "tick_with_data",
        owner,
        "notify_with_data",
        VariantArray::new_shared(),
        0,
      )
      .unwrap();
  }

  #[export]
  fn notify(&mut self, owner: &Label) {
    self.times_received += 1;
    let msg = format!("Received signal \"tick\" {} times", self.times_received);

    owner.set_text(msg);
  }

  #[export]
  fn notify_with_data(&mut self, owner: &Label, data: Variant) {
    let msg = format!(
      "Received signal \"tick_with_data\" with data {}",
      data.try_to_u64().unwrap()
    );

    owner.set_text(msg);
  }
}
