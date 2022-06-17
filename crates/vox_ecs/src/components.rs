use image::DynamicImage;

#[derive(Debug, Clone, Copy)]
pub struct Component2D{
    pub position: [f32;2],
    pub rotation: f32
}

impl Component2D{
    pub fn new() -> Self{
        Self{position:[0.,0.], rotation: 0.}
    }
}

#[derive(Debug)]
pub struct Sprite2D{
    pub image: DynamicImage
}

impl Sprite2D{
    pub fn new(img: DynamicImage) -> Self{
        Self { image: img }
    }
}