use crate::cube::core::Cube;
use crate::globals::MOVE_LIST;
use crate::solver::core::Solver;

impl Solver {

    fn search (&self, cube : &mut Cube) {
    }
    fn ida_star(&self, cube : &mut Cube) {

    }
    pub fn solve(&mut self, cube : &mut Cube) {
        cube.apply_move(0, true);
        cube.edge_heuristics();
    }
}