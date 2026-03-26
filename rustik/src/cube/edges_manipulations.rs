use crate::cube::core::Cube;
use crate::globals::AFFECTED_EDGE_SLOTS;
impl Cube {
    pub fn apply_move_on_edges(&mut self , move_face : usize, move_is_clockwise : bool ){
        let mask : i64 = 0b11111;
        let slots = AFFECTED_EDGE_SLOTS[move_face];
        let move_mask = Self::build_move_mask(slots);
        let flipping_move = (move_face <= 1) as i64;
        let chunks : [i64; 4] = [
            ((self.edges >> (slots[0] * 5)) & mask) ^ flipping_move,
            ((self.edges >> (slots[1] * 5)) & mask) ^ flipping_move,
            ((self.edges >> (slots[2] * 5)) & mask) ^ flipping_move,
            ((self.edges >> (slots[3] * 5)) & mask) ^ flipping_move
        ];
        self.edges &= !move_mask;

        self.edges |= if move_is_clockwise{
            self.apply_clockwise(&chunks, &slots)
        }else {
            self.apply_counter_clockwise(&chunks, &slots)
        };
    }
}