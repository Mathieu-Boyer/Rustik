pub struct Cube {
    pub edges : i64
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
        Cube {
            edges : compute_solved_edges
        }
    }
}

