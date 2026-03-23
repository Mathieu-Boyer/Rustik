use crate::cube::core::Cube;

pub mod cube;

fn main() {
    println!("Hello, world!");


    let cube = Cube::new();

    cube.display();
    cube.show_slots();
}
