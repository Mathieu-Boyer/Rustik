use crate::cube::core::Cube;
pub mod cube;

fn main() {
    let mut cube = Cube::new();
    cube.display();
    cube.apply_move(0, true);
    cube.apply_move(0, false);
    cube.show_corners_slots();
}
