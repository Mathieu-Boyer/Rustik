use crate::cube::core::Cube;

impl Cube {
    pub fn switch_slots(&mut self, slot_id1 : i64, slot_id2 : i64) {
        let mask : i64 = 0b11111;
        let combined_masks : i64 = (mask << (5 * slot_id1)) | (mask << (5 * slot_id2));
        let chunk_at_slot1 : i64 = (self.edges >> (slot_id1 * 5)) & mask;
        let chunk_at_slot2 : i64 = (self.edges >> (slot_id2 * 5)) & mask;

        self.edges &= !combined_masks;
        self.edges |= (chunk_at_slot2 << (5 * slot_id1)) | (chunk_at_slot1 << (5 * slot_id2));
    }
}