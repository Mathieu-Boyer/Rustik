use crate::cube::core::Cube;
use crate::cube::globals::AFFECTED_CORNERS_SLOTS;

impl Cube {



    fn apply_move_on_corners_anti_clockwise(&mut self , chunks: [i64; 4], slots : [i64; 4]) {
        self.edges |= (chunks[3] << (5 * slots[0])) | (chunks[0] << (5 * slots[1])) | (chunks[1] << (5 * slots[2])) | (chunks[2] << (5 * slots[3]));
    }
    fn apply_move_on_corners_clockwise(&mut self , chunks: [i64; 4], slots : [i64; 4]) {
        self.corners |= (chunks[1] << (5 * slots[0])) | (chunks[2] << (5 * slots[1])) | (chunks[3] << (5 * slots[2])) | (chunks[0] << (5 * slots[3]));
    }
    pub fn apply_move_on_corners (&mut self , move_face : usize, move_is_clockwise : bool ) {
        let mask = 0b11111;
        let slots = AFFECTED_CORNERS_SLOTS[move_face];
        let move_mask = Self::build_move_mask(slots);
        let chunks = [
            self.corners >> (5 * slots[0]) & mask,
            self.corners >> (5 * slots[1]) & mask,
            self.corners >> (5 * slots[2]) & mask,
            self.corners >> (5 * slots[3]) & mask,
        ];

        self.corners &= !move_mask;

        if move_is_clockwise {
            self.apply_move_on_corners_clockwise(chunks, slots);
        }else {
            self.apply_move_on_corners_anti_clockwise(chunks, slots);
        }

    }
}