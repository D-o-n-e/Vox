use Vox::*;

#[derive(Debug, Clone, Copy)]
pub struct Player;

// Overidding methods
impl Script for Player{
    fn new() -> Self {
        Player
    } 
    fn input(&self, owner: &mut Components, event: InputEvent) {
        let mut own = owner.get_component2D();
        if event.is_pressed("W"){
            own.position[1] += 0.02;
        }
        if event.is_pressed("A"){
            own.position[0] -= 0.02;
        }
        if event.is_pressed("S"){
            own.position[1] -= 0.02;
        }
        if event.is_pressed("D"){
            own.position[0] += 0.02;
        }
        owner.set_component2D(own);
    }
}

// Run
fn main(){
    let mut world = World::new();
    let component = Component2D::new(0);
    world.add_component::<Player>(Components::Component2D(component));
    Engine::start(world);
}