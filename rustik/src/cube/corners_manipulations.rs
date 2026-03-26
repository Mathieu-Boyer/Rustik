use crate::cube::core::Cube;
use crate::globals::AFFECTED_CORNERS_SLOTS;

impl Cube {
    pub fn build_move_mask(affected_slots: [i64; 4]) -> i64 {
        let mask : i64 = 0b11111;
        let mut move_mask = 0;
        for i in 0..4{
            move_mask |=  mask << (5 * affected_slots[i])
        }

        move_mask
    }

    pub fn apply_deltas(chunks : &mut[i64; 4]){
        let orientation_mask = 0b11;
        let meow  = [2,1,2,1];
        for i in 0..4 {
            let mut orientation = chunks[i] & orientation_mask;
            chunks[i] &= !orientation_mask;
            orientation = (orientation + meow[i]) % 3;
            chunks[i] |= orientation;
        }
    }

    pub fn apply_move_on_corners (&mut self , move_face : usize, move_is_clockwise : bool ) {
        let mask = 0b11111;
        let slots = AFFECTED_CORNERS_SLOTS[move_face];
        let move_mask = Self::build_move_mask(slots);
        let mut chunks = [
            self.corners >> (5 * slots[0]) & mask,
            self.corners >> (5 * slots[1]) & mask,
            self.corners >> (5 * slots[2]) & mask,
            self.corners >> (5 * slots[3]) & mask,
        ];
        if move_face > 1{
            Self::apply_deltas(&mut chunks);
        }

        self.corners &= !move_mask;

        self.corners |= if move_is_clockwise{
            self.apply_clockwise(&chunks, &slots)
        }else {
            self.apply_counter_clockwise(&chunks, &slots)
        };

    }
}