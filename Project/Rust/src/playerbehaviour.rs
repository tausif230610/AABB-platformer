use godot::prelude::*;
use godot::engine::Node2D;

use crate::godotbox::GodotBox;

#[derive(GodotClass)]
#[class(init,base=Node2D)]
struct PlayerBehaviour{
    #[export]
    speed:i32,
    #[export]
    #[init(default=None)]
    master:Option<Gd<GodotBox>>,
    base:Base<Node2D>
}
#[godot_api]
impl INode2D for PlayerBehaviour {
fn process(&mut self,_delta:f64){

}
}
#[godot_api]
impl PlayerBehaviour {
    #[func]
    fn master_shape_add_top(&mut self,amount:i32){
        let mut additionsuccess=false;
        if let Some(master) = self.master.as_mut() {
            master.bind_mut().add_top(amount);
            additionsuccess=true;
        }
        if additionsuccess{
            let mut pos=self.base_mut().get_position();
            pos.y+=(amount/2) as f32;
            self.base_mut().set_position(pos);
        }
    }
    #[func]
    fn master_shape_add_bottom(&mut self,amount:i32){
        let mut additionsuccess=false;
        if let Some(master) = self.master.as_mut() {
            master.bind_mut().add_bottom(amount);
            additionsuccess=true;
        }
        if additionsuccess{
            let mut pos=self.base_mut().get_position();
            pos.y-=(amount/2) as f32;
            self.base_mut().set_position(pos);
        }
    }
    #[func]
    fn master_shape_add_left(&mut self,amount:i32){
        let mut additionsuccess=false;
        if let Some(master) = self.master.as_mut() {
            master.bind_mut().add_left(amount);
            additionsuccess=true;
        }
        if additionsuccess{
            let mut pos=self.base_mut().get_position();
            pos.x+=(amount/2) as f32;
            self.base_mut().set_position(pos);
        }
    }
    #[func]
    fn master_shape_add_right(&mut self,amount:i32){
        let mut additionsuccess=false;
        if let Some(master) = self.master.as_mut() {
            master.bind_mut().add_right(amount);
            additionsuccess=true;
        }
        if additionsuccess{
            let mut pos=self.base_mut().get_position();
            pos.x-=(amount/2) as f32;
            self.base_mut().set_position(pos);
        }
    }
    
}