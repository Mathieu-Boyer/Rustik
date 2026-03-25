use crate::cube::core::Cube;
use crate::cube::globals::AFFECTED_EDGE_SLOTS;
impl Cube {

    fn build_move_mask(affected_slots: [i64; 4]) -> i64 {
        let mask : i64 = 0b11111;
        let mut move_mask = 0;
        for i in 0..4{
            move_mask |=  mask << (5 * affected_slots[i])
        }

        move_mask
    }



    fn apply_clockwise(&mut self, chunks: &[i64; 4], slots : &[i64; 4]){
        self.edges |= (chunks[3] << (5 * slots[0]))
            | (chunks[0] << (5 * slots[1]))
            | (chunks[1] << (5 * slots[2]))
            | (chunks[2] << (5 * slots[3]));
    }
    fn apply_counter_clockwise(&mut self, chunks: &[i64; 4], slots : &[i64; 4]){
        self.edges |= (chunks[1] << (5 * slots[0]))
            | (chunks[2] << (5 * slots[1]))
            | (chunks[3] << (5 * slots[2]))
            | (chunks[0] << (5 * slots[3]));
    }
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

        if move_is_clockwise{
            self.apply_clockwise(&chunks, &slots)
        }else{
            self.apply_counter_clockwise(&chunks, &slots)   
        }
    }
}