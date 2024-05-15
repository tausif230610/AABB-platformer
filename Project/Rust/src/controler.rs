use godot::prelude::*;
use godot::engine::Node;
const _PLAYER_ROLL_PIXEL_DUCK_AMOUNT: i32=5;
#[derive(GodotClass)]
#[class(init,base=Node)]
struct Controler{
    direction:i8,
    base:Base<Node>
}
#[godot_api]
impl INode for Controler {
    fn process(&mut self, _: f64,) {
        let input_handler=Input::singleton();
        let dir=input_handler.get_action_strength("ui_right".into()) as i8 - input_handler.is_action_pressed("ui_left".into()) as i8;
        self.direction=dir;
    }
}