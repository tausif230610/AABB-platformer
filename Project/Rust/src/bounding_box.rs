
use godot::prelude::*;
use godot::classes::{Node2D,INode2D};


#[derive(GodotClass)]
#[class(base=Node2D,tool)]

struct BoundingBox{
    #[export]
    size:Vector2i,
    #[export]
    color:Color,
    base:Base<Node2D>
}


#[godot_api]
impl INode2D for BoundingBox {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            size:Vector2i::ZERO,
            color:Color::SKY_BLUE,
            base
        }
    }
    
    fn process(&mut self,_delta:f64){
        let position=self.base().get_position().cast_int();
        self.base_mut().set_position(position.cast_float());
        self.base_mut().queue_redraw();
    }
    fn draw(&mut self){
        let color=self.color;

                let size=self.size.cast_float();
                self.base_mut().draw_rect(Rect2::from_components(-(size.x/2.0), -(size.y/2.0), size.x, size.y), color);
            }
        }

#[godot_api]
impl BoundingBox {
    #[signal]
    fn has_colided(&mut other:Gd<BoundingBox>);
    #[func]
    fn collision_checker(&self,other:Gd<BoundingBox>)->bool{
        let sleft=self.base().get_position().x  - (self.size.x/2)         as f32;
        let sright=self.base().get_position().x + (self.size.x/2)         as f32;
        let sup=self.base().get_position().y    - (self.size.y/2)         as f32;
        let sdown=self.base().get_position().y  + (self.size.y/2)         as f32;
        let oleft=other.get_position().x        - (other.bind().size.x/2) as f32;
        let oright=other.get_position().x       + (other.bind().size.x/2) as f32;
        let oup=other.get_position().y          - (other.bind().size.y/2) as f32;
        let odown=other.get_position().y        + (other.bind().size.y/2) as f32;
        sleft<=oright && oleft<=sright && sup<=odown && oup<=sdown
        
    }

}