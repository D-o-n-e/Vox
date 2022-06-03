use Vox::*;

// Test inheritance from api
#[extends(Node2D)]
pub struct Player;

// Overidding methods
impl Script for Player{
    fn _ready(&self){
        // "test" method is placeholder
        println!("{:?}", self.test());
    }
}

// All placeholders, this is ECS example
fn main(){
    let player = Player{position: [0.,0.], rotation: 24.};
    player._ready();
}