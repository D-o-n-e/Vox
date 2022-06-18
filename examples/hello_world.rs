use Vox::*;

#[derive(Debug, Clone, Copy)]
pub struct Player;

// Overidding methods
impl Script for Player{
    fn new() -> Self {
        Player
    }
    fn start(&self, owner: &mut Components) {
        println!("Hello, World");
    }
}

// Run
fn main(){
    let mut world = World::new();
    let component = Component2D::new(0);
    world.add_component::<Player>(Components::Component2D(component));
    Engine::start(world);
}
