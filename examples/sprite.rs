use Vox::*;

#[derive(Debug)]
pub struct Sprite;

// Overidding methods
impl Script for Sprite{
    fn new() -> Self {
        Sprite
    }
    fn start(&self, owner: &Components, world: &World){
        println!("Sprite Class!");  
    }   
}

// Run ECS
fn main(){
    let mut world = World::new();
    world.add_component::<Sprite>(instance(load("examples/icon.png").unwrap()).unwrap());
    Engine::start(world);
}