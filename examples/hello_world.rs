use Vox::*;

#[derive(Debug)]
pub struct Player;

// Overidding methods
impl Script<Component2D> for Player{
    fn new() -> Self {
        Player
    }
    fn start(&self, owner: Component2D){
        println!("Hello, World");  
    }
}

// Custom Methods
impl Player{
    fn add(&self, a: i32, b: i32) -> i32{
        a + b
    }
}

// All placeholders, this is ECS example
fn main(){
    start::<Player, Component2D>(Component2D::new([0.,0.], 1.));
}