use Vox::*;

#[derive(Debug)]
pub struct Player;

// Overidding methods
impl Script for Player{
    fn new() -> Self {
        Player
    }
    fn start(&self, mut owner: &Components){
        println!("Hello World");  
    }
    
}

// Run ECS
fn main(){
    let mut world = World::new();
    world.add_component::<Player>(Component2D{position:[0.,0.], rotation:0.});
    world.start();
}