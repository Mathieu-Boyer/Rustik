use crate::cube::core::Cube;
use crate::globals::{Move, MOVE_LIST, OPPOSITE_FACE};
use crate::solver::core::Solver;
use crate::solver::enums::SearchReturnValues;

impl Solver {

    fn search (&self, cube : &mut Cube, g: usize, threshold : usize, moves : &mut Vec<Move>, depth : &usize) -> SearchReturnValues {

        let og_h = cube.heuristics();
        let f = g + og_h;

        if f > threshold {
            return SearchReturnValues::ThresholdExceeded(f);
        }

        if cube.is_solved(){
            return SearchReturnValues::SolutionFound(moves.clone());
        }

        let mut min_threshold = usize::MAX;

        for m in MOVE_LIST {

            if moves.len() > 0{
                let last = moves.last().unwrap();
                if last.face == m.face{ continue }
                if last.face == OPPOSITE_FACE[m.face] { continue }
            }

            // if moves.len() >= 2 {
            //     let last = &moves[moves.len() - 1];
            //     let second_last = &moves[moves.len() - 2];
            //
            //     // Already handled: same face as last
            //     if last.face == m.face { continue }
            //
            //     // Already handled: opposite face after last
            //     if last.face == OPPOSITE_FACE[m.face as usize] { continue }
            //
            //     // NEW: e.g. R L R — the two R's could've been a single move
            //     // If second_last is opposite to last, and m == second_last face, skip
            //     if second_last.face == m.face && last.face == OPPOSITE_FACE[m.face as usize] {
            //         continue;
            //     }
            // }

            cube.apply_move(&m.face, &m.is_clockwise, &m.num);

                moves.push(m.clone());
                let result = self.search(cube, g + 1, threshold, moves, &(*depth + 1));
                match result {
                    SearchReturnValues::SolutionFound(moves) => return SearchReturnValues::SolutionFound(moves),
                    SearchReturnValues::ThresholdExceeded(new_threshold) => min_threshold = min_threshold.min(new_threshold),
                    SearchReturnValues::NoSolution => {},
                };

                moves.pop();

            cube.undo(&m.face, &m.is_clockwise, &m.num);
        }

        if min_threshold == usize::MAX {
            SearchReturnValues::NoSolution
        }else {
            SearchReturnValues::ThresholdExceeded(min_threshold)
        }
    }
    fn ida_star(&self, cube: &mut Cube) -> Vec<Move> {
        let mut threshold = cube.heuristics();
        let mut it = 0;
        loop {
            println!("{}", it);
            let result = self.search(cube, 0, threshold, &mut vec![], &0);
            match result {
                SearchReturnValues::SolutionFound(moves) => {
                    println!("Solution was found with size {}", moves.len());
                    return moves;
                } ,
                SearchReturnValues::ThresholdExceeded(new_threshold) =>  threshold = new_threshold ,
                SearchReturnValues::NoSolution => {
                    println!("No solutions found") ;
                    return vec![]
                }, // maybe i should send a panic here ? idk
            };

            it += 1;
        }
    }


    pub fn solve(&mut self, cube : &mut Cube) {
        let moves = self.ida_star(cube);

        for m in moves {
            println!("{}", m.name);
        }
    }
}