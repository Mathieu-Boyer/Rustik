pub struct Cube {
    pub edges : i64,
    pub corners : i64,

    pub solved_edges : i64,
    pub solved_corners : i64
}


impl Cube {
    pub fn new() -> Cube {
        let mut compute_solved_edges: i64 = 0;
        for i  in 0..12{
            let slot = i << 1;
            let is_flipped = 0;
            let chunk = slot | is_flipped;

            compute_solved_edges |= chunk << (5 * i);
        }

        let mut compute_solved_corners: i64 = 0;
        for i in 0..8{
            let slot =  i << 2;
            let orientation = 0;

            let chunk = slot | orientation;

            compute_solved_corners |= chunk << (5 * i)
        }


        Cube {
            edges : compute_solved_edges,
            corners : compute_solved_corners,

            solved_edges : compute_solved_edges,
            solved_corners : compute_solved_corners
        }
    }


    pub fn is_solved(&self) -> bool{
        (self.edges == self.solved_edges) && (self.corners == self.solved_corners)
    }
}

