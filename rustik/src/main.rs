use crate::cube::core::Cube;

pub mod cube;

fn main() {
    println!("Hello, world!");


    let mut cube = Cube::new();

    cube.display();
    cube.show_slots();

    cube.switch_slots(6, 11);

    cube.switch_slots(6, 11);
    
}
