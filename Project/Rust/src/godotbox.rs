use godot::prelude::*;
use godot::engine:: Node2D;

#[derive(GodotClass)]
#[class(tool,base=Node2D)]
pub struct GodotBox{
#[export]
area:Vector2i,
#[export]
color:Color,
base:Base<Node2D>
}
#[godot_api]
impl INode2D for GodotBox{
    fn init(base:Base<Node2D>)->Self{
        Self{
            area:Vector2i::ZERO,
            color:Color::WHITE,

            base
        }
    }
    fn draw(&mut self){
        
        let area=Vector2::from_vector2i(self.area);
        let color=self.color;
        
        self.base_mut().draw_rect(Rect2 { position: Vector2::new((-((area.x)/2.0)) as f32, (-((area.y)/2.0)) as f32), size: area }, color);
        
    }
    fn process(&mut self,_delta:f64){
        self.base_mut().queue_redraw();
        let position=Vector2i::from_vector2(self.base_mut().get_position());
        self.base_mut().set_position(Vector2::from_vector2i(position));
    }
}
#[godot_api]
impl GodotBox {
    #[func]
    pub fn get_top(&mut self)->i32{
        let halfysize=self.area.y/2;
        self.base_mut().get_position().y as i32-(halfysize)
    }
    #[func]
    pub fn get_bottom(&mut self)->i32{
        let halfysize=self.area.y/2;
        self.base_mut().get_position().y as i32+(halfysize)
    }
    #[func]
    pub fn get_left(&mut self)->i32{
        let halfxsize=self.area.x/2;
        self.base_mut().get_position().x as i32-(halfxsize)
    }
    #[func]
    pub fn get_right(&mut self)->i32{
        let halfxsize=self.area.x/2;
        self.base_mut().get_position().x as i32+(halfxsize)
    }
    #[func]
    pub fn add_top(&mut self,value:i32){
        let mut top=self.get_top();
        let bottom=self.get_bottom();
        top-=value;
        let posy=(bottom+top)/2;
        let posx=self.base_mut().get_position().x ;
        self.base_mut().set_position(Vector2 { x: posx, y: posy as f32 });
        self.area.y+=value;
    }
    #[func]
    pub fn add_bottom(&mut self,value:i32){

        let  top=self.get_top();
        let mut bottom=self.get_bottom();
        bottom+=value;
        let posy=(bottom+top)/2;
        let posx=self.base_mut().get_position().x ;
        self.base_mut().set_position(Vector2 { x: posx , y: posy as f32 });
        self.area.y+=value;
    }
    #[func]
    pub fn add_left(&mut self,value:i32){
        let mut left=self.get_left();
        let right=self.get_right();
        left-=value;
        let posy=self.base_mut().get_position().y;
        let posx=(left+right)/2 ;
        self.base_mut().set_position(Vector2 { x: posx as f32, y: posy  });
        self.area.x+=value;

    }
    #[func]
    pub fn add_right(&mut self,value:i32){
        let  left=self.get_left();
        let mut right=self.get_right();
        right+=value;
        let posy=self.base_mut().get_position().y;
        let posx=(left+right)/2 ;
        self.base_mut().set_position(Vector2 { x: posx as f32, y: posy  });
        self.area.x+=value;
    }
    #[func]
    pub fn has_colided(&mut self,&mut other:Gd<GodotBox>)->bool{
        (self.get_top()<=other.bind_mut().get_bottom()||self.get_bottom()>=other.bind_mut().get_top())&&(self.get_right()>=other.bind_mut().get_left()||self.get_left()<=other.bind_mut().get_right())
    }
}