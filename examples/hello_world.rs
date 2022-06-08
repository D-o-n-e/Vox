use resources::*;
use Vox::*;

#[derive(Debug)]
pub struct Player1{
    time: i32
}
pub struct Player2;

// Overidding methods
impl Script for Player1{
    fn new() -> Self {
        Player1{time: 10}
    }
    fn start(&self, mut owner: Components){
        println!("Player1");  
    }
    
}

// Beta Resources
impl Script for Player2{
    fn new() -> Self {
        Player2
    }
    fn start(&self, mut owner: Components){
        println!("Player2");  
    }
    fn resources(&self, resources: &Vec<i32>) {
        // Use Resources
    }
    
}

// Run ECS
fn main(){
    let mut world = World::new();
    world.add_resource(10);
    world.add_component::<Player1>(Components::Component2D{position:[1.,0.]});
    world.add_component::<Player2>(Components::Component2D{position:[10.,0.]});
    world.start();
}