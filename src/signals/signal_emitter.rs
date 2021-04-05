use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
// register_with attribute can be used to specify custom register function for node signals and properties
#[register_with(Self::register_signals)]
pub struct SignalEmitter {
  timer: f64,
  data: i64,
}

trait EmitSignal<C> {
  fn new(owner: &Node) -> C;
  fn register_signals(builder: &ClassBuilder<C>);
}

impl EmitSignal<SignalEmitter> for SignalEmitter {
  fn register_signals(builder: &ClassBuilder<Self>) {
    builder.add_signal(Signal {
      name: "tick",
      args: &[],
    });

    builder.add_signal(Signal {
      name: "tick_with_data",
      // Argument list used by the editor for GUI and generation of GDScript handlers. It can be omitted if the signal is only used from code.
      args: &[SignalArgument {
        name: "data",
        default: Variant::from_i64(100),
        export_info: ExportInfo::new(VariantType::I64),
        usage: PropertyUsage::DEFAULT,
      }],
    });
  }

  fn new(_owner: &Node) -> Self {
    SignalEmitter {
      timer: 0.0,
      data: 100,
    }
  }
}

#[methods]
impl SignalEmitter {
  #[export]
  fn _process(&mut self, owner: &Node, delta: f64) {
    if self.timer < 1.0 {
      self.timer += delta;
      return;
    }
    self.timer = 0.0;
    self.data += 1;

    if self.data % 2 == 0 {
      owner.emit_signal("tick", &[]);
    } else {
      owner.emit_signal("tick_with_data", &[Variant::from_i64(self.data)]);
    }
  }
}
