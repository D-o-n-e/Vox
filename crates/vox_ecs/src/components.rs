use image::DynamicImage;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Component2D{
    pub id: i64,
    pub position: [f32;2],
    pub rotation: f32
}

impl Component2D{
    pub fn new(id: i64) -> Self{
        Self{id: id, position:[0.,0.], rotation: 0.}
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sprite2D{
    //pub image: DynamicImage
}

impl Sprite2D{
    pub fn new(img: DynamicImage) -> Self{
        Self {  }
        //Self { image: img }
    }
}