use image::{io::Reader as ImageReader, DynamicImage};
use std::collections::HashMap;
mod components;
use winit::event::*;
pub use components::*;

pub type InputEvent = VirtualKeyCode;
pub trait Input{
    fn is_pressed(self, key: &str) -> bool;
}

pub trait Script{
    fn new() -> Self where Self: Sized;
    fn start(&self, owner: &mut Components){}
    fn update(&self, owner: &mut Components){}
    fn input(&self, owner: &mut Components, event: InputEvent){}
    fn resources(&self, resources: &Vec<i32>){}
}

impl Input for VirtualKeyCode{
    fn is_pressed(self, key: &str) -> bool {
        key == vkk_to_str(self)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Components{
    Component2D(Component2D),
    Sprite2D(Sprite2D)
}



impl Components{
    pub fn get_component2D(self) -> Component2D{
        match self{
            Components::Component2D(data) => {
                data
            },
            _ => {Component2D::new(-1)}
        }
    }
    pub fn set_component2D(&mut self, comp: Component2D){
        match self{
            Components::Component2D(data) => {
                *data = comp;
            }
            _ => {}
        }
    }
}

pub enum Objects{
    Texture{
        data: DynamicImage
    }
}



pub fn instance(obj: Objects) -> Result<Components, &'static str>{
    match obj{
        Objects::Texture { data } => Ok(Components::Sprite2D(Sprite2D::new(data))),
        _ => Err("Cannot instance")
    }
}

pub fn load(path: &str) -> Result<Objects, &str>{
    if path.ends_with("png"){
        let img = ImageReader::open(path)
            .unwrap()
            .decode()
            .unwrap();
        return Ok(Objects::Texture { data: img });
    }
    Err("Cannot load that file")
    
    
}

pub struct World
{
    pub resources: Vec<i32>,
    pub components: Vec<(Box<dyn Script>,Components)>, 
}

impl World
{
    pub fn new() -> Self
    {
        Self{resources: vec![], components: vec![]}
    }

    pub fn add_component<T>(&mut self, comp: Components)
    where
        T: Script + 'static
    {
        self.components.append(&mut vec![(Box::new(T::new()),comp)]);
    }
    pub fn add_resource(&mut self, res: i32){
        self.resources.append(&mut vec![res]);
    }
}


fn vkk_to_str(ch: VirtualKeyCode) -> &'static str{
    match ch{
        VirtualKeyCode::W => "W",
        VirtualKeyCode::A => "A",
        VirtualKeyCode::S => "S",
        VirtualKeyCode::D => "D",
        _ => ""
    }
}