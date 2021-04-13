extends RigidBody2D

signal clicked(node)
signal dropped(node)

var held = false

func _input_event(_viewport, event, _shape_idx):
  if event is InputEventMouseButton:
    match [event.button_index, event.pressed]:
      [BUTTON_LEFT, true]:
        emit_signal("clicked", self)
      [BUTTON_LEFT, false]:
        emit_signal("dropped", self)

func _physics_process(_delta):
  if held:
    global_transform.origin = get_global_mouse_position()

func pickup():
  if held:
    return
  mode = RigidBody2D.MODE_STATIC
  held = true

func drop(impulse=Vector2.ZERO):
  if held:
    mode = RigidBody2D.MODE_RIGID
    apply_central_impulse(impulse)
    held = false

