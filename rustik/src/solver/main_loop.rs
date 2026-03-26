use crate::cube::core::Cube;
use crate::solver::core::Solver;

impl Solver {

    fn main_loop (&mut self, cube : &mut Cube) {

    }
    fn ida_star() {
        
    }
    pub fn solve(&mut self, cube : &mut Cube) {
        cube.apply_move(0, true);
        cube.edge_heuristics();
    }
}