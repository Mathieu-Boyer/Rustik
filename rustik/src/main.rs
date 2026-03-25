use crate::cube::core::Cube;

pub mod cube;

fn main() {
    println!("Hello, world!");


    let mut cube = Cube::new();

    cube.display();

    // cube.apply_move_on_edges(1, true);
    cube.apply_move_on_corners(0, true);
    // cube.apply_move_on_edges(5, false);
    // cube.apply_move_on_edges(0, false);
    // cube.apply_move_on_edges(0, false);

     // cube.show_slots();

    cube.show_corners_slots();
}
