use image::{io::Reader as ImageReader, DynamicImage};
pub trait Script{
    fn new() -> Self where Self: Sized;
    fn start(&self, owner: &Components){}
    fn update(&self){}
    fn resources(&self, resources: &Vec<i32>){}
}

#[derive(Debug)]
pub enum Components{
    Component2D{
        position: [f32;2],
        rotation: f32
    },
    Sprite2D{
        image: DynamicImage
    }
}

pub enum Objects{
    Texture{
        data: DynamicImage
    }
}

impl Components{
    pub fn get_position(self) -> [f32;2]{
        match self{
            Components::Component2D{position, rotation} => position,
            _ => {panic!("Error")}
        }
    }
    pub fn set_position(&mut self, pos: [f32;2]){
        match self{
            Components::Component2D{position, rotation} => *position = pos,
            _ => {panic!("Error")}
        }
    }
}

pub struct World
{
    resources: Vec<i32>,
    components: Vec<Components>,
    classes: Vec<Box<dyn Script>>
}

pub fn instance(obj: Objects) -> Result<Components, &'static str>{
    match obj{
        Objects::Texture { data } => Ok(Components::Sprite2D { image: data }),
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

impl World
{
    pub fn new() -> Self
    {
        Self{resources: vec![], components: vec![], classes: vec![]}
    }

    pub fn start(&self)
    {
        for i in 0..self.classes.len(){
            self.classes[i].start(&self.components[i]);   
        }
        
        // c.start(self.components[0].class);
        loop{
            for c in &self.classes{
                c.resources(&(self.resources))
            }
        }
    }
    pub fn add_component<T>(&mut self, comp: Components)
    where
        T: Script + 'static
    {
        self.classes.append(&mut vec![Box::new(T::new())]);
        self.components.append(&mut vec![comp]);
        
        //.start(Components::Component2D{position: [0.,0.]})
    }
    pub fn add_resource(&mut self, res: i32){
        self.resources.append(&mut vec![res]);
    }
}
