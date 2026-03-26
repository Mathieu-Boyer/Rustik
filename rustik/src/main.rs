use crate::cube::core::Cube;
use crate::solver::core::Solver;

pub mod cube;
pub mod globals;
pub mod solver;

fn main() {
    let mut cube = Cube::new();
    let mut solver = Solver::new();

    solver.solve(&mut cube);

    cube.display();
    
}
