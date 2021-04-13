extends Node2D

var held_object = null


func _ready():
  for node in get_tree().get_nodes_in_group("pickable"):
      node.connect("clicked", self, "_on_pickable_clicked")
      node.connect("dropped", self, "_on_pickable_dropped")

func _on_pickable_clicked(object):
  if !held_object:
    held_object = object
    held_object.pickup()

func _on_pickable_dropped(_object):
  pass

func _unhandled_input(event: InputEvent):
  if event is InputEventMouseButton and event.button_index == BUTTON_LEFT:
    if held_object and !event.pressed:
      # held_object.drop(Input.get_last_mouse_speed())
      held_object.drop()
      held_object = null
