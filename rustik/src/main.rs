use crate::cube::core::Cube;
use crate::solver::core::Solver;

pub mod cube;
pub mod globals;
pub mod solver;

fn main() {
    let mut cube = Cube::new();
    let mut solver = Solver::new();


    cube.apply_move(&0, &true, &1);
    cube.apply_move(&2, &false, &1);
    cube.apply_move(&4, &false, &1);
    cube.apply_move(&2, &false, &1);
    cube.apply_move(&0, &true, &1);
    cube.apply_move(&0, &true, &1);
    cube.apply_move(&5, &true, &1);
    cube.apply_move(&1, &false, &1);
    cube.apply_move(&3, &true, &1);
    solver.solve(&mut cube);

    cube.display();
    
}
