use Vox::*;

#[derive(Debug, Clone, Copy)]
pub struct Player;

// Overidding methods
impl Script for Player{
    fn new() -> Self {
        Player
    }
    fn start(&self, owner: &Components, world: &World) {
        println!("Position: {:?}", owner.get_component2D().position);  
    } 
}

// Run ECS
fn main(){
    let mut world = World::new();
    let mut component = Component2D::new();
    component.position = [10.,0.];
    world.add_component::<Player>(Components::Component2D(component));
    Engine::start(world);
}
