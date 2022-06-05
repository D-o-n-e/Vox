
#[derive(Debug)]
pub struct Component2D{
    pub position: [f32; 2],
    pub rotation: f32
}
pub trait Component{
    fn new(position: [f32;2], rotation: f32) -> Self;
}

impl Component for Component2D{
    fn new(position: [f32;2], rotation: f32) -> Self{
        Self{position: position, rotation: rotation}
    }
}
// pub struct Engine();

// impl Engine{
//     pub fn start(self){

//     }
// }

pub fn start<C, T>(component: T)
where
    C: Script<T>
{
    C::new().start(component);
}

pub trait Script<T>{
    fn new() -> Self;
    fn start(&self, owner: T){}
    fn update(&self){}
}