use crate::cube::core::Cube;

impl Cube {

    pub fn undo(&mut self, move_face: usize, move_is_clockwise: bool) {
        self.apply_move( move_face, !move_is_clockwise)
    }
    
    pub fn apply_move(&mut self, move_face: usize, move_is_clockwise: bool){
        self.apply_move_on_edges(move_face, move_is_clockwise);
        self.apply_move_on_corners(move_face, move_is_clockwise);
    }
    pub fn apply_clockwise(&self, chunks: &[i64; 4], slots : &[i64; 4]) -> i64{
        (chunks[3] << (5 * slots[0])) | (chunks[0] << (5 * slots[1])) | (chunks[1] << (5 * slots[2])) | (chunks[2] << (5 * slots[3]))
    }
    pub fn apply_counter_clockwise(&self, chunks: &[i64; 4], slots : &[i64; 4]) -> i64{
        (chunks[1] << (5 * slots[0])) | (chunks[2] << (5 * slots[1])) | (chunks[3] << (5 * slots[2])) | (chunks[0] << (5 * slots[3]))
    }
}