extends BoundingBox
func _process(_delta):
	position=get_global_mouse_position()
	position=Vector2(Vector2i(position))
