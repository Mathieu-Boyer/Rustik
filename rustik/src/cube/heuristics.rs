use crate::cube::core::Cube;

impl Cube {
    pub fn corner_heuristics(&self) -> i64{
        let mut misplaced_corners = 0;
        let mut misoriented_corners = 0;


        let mask = 0b11111;
        for i in 0..8 {
            let chunk = (self.corners >> (5 * i)) & mask;
            if i != (chunk >> 2) { misplaced_corners += 1}
            if (chunk & 0b11) != 0 {misoriented_corners += 1}
        }

        (misplaced_corners).max(misoriented_corners)
    }
    pub fn edge_heuristics(&self) -> i64{
        let mut misplaced_edges = 0;
        let mut flipped_edges = 0;


        let mask = 0b11111;
        for i in 0..12 {
            let chunk = (self.edges >> (5 * i)) & mask;
            if i != (chunk >> 1) { misplaced_edges += 1}
            if (chunk & 1) == 1 {flipped_edges += 1}
        }

        (misplaced_edges).max(flipped_edges)
    }

    pub fn heuristics(&self) -> usize{
        (self.edge_heuristics() + (self.corner_heuristics())) as usize
    }
}