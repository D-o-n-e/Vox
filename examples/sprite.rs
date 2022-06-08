use Vox::*;

#[derive(Debug)]
pub struct Player;

// Overidding methods
impl Script for Player{
    fn new() -> Self {
        Player
    }
    fn start(&self, owner: &Components){
        println!("Hello World");  
    }
    
}

// Run ECS
fn main(){
    let mut world = World::new();
    world.add_component::<Player>(instance(load("examples/icon.png").unwrap()).unwrap());
    world.start();
}