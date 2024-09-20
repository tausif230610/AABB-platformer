extends BoundingBox
func  _process(_delta:float):
	print(self.collision_checker($"../BoundingBox"))
