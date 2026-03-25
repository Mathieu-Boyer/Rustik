use crate::cube::core::Cube;

impl Cube{
    pub fn display (&self){
        println!("{:064b}", self.edges);
    }

    pub fn slot_from_int(&self, slot: i64) -> String {
        String::from(match slot {
            0 => "UF",
            1 => "UR",
            2 => "UB",
            3 => "UL",
            4 => "FR",
            5 => "BR",
            6 => "BL",
            7 => "FL",
            8 => "DF",
            9 => "DR",
            10 => "DB",
            11 => "DL",
            _ => {""}
        })
    }
    pub fn show_slots(&self) {
        for i in 0..12{
            let mask : i64 = 0b11111;
            let chunk =  (self.edges >> (i * 5)) & mask ;
            let slot = chunk >> 1;
            let is_flipped = chunk & 1;

            let color = if slot == i { "\x1b[32m"} else {"\x1b[31m"};

            print!("{}| {}{}[{}] \x1b[0m|",i,color, self.slot_from_int(i) , self.slot_from_int(slot));


            print!(" FlipState : {} |\n", is_flipped);
        }
    }
}


// pub fn switch_slots(&mut self, slot_id1 : i64, slot_id2 : i64) {
//     let mask : i64 = 0b11111;
//     let combined_masks : i64 = (mask << (5 * slot_id1)) | (mask << (5 * slot_id2));
//     let chunk_at_slot1 : i64 = (self.edges >> (slot_id1 * 5)) & mask;
//     let chunk_at_slot2 : i64 = (self.edges >> (slot_id2 * 5)) & mask;
//
//     self.edges &= !combined_masks;
//     self.edges |= (chunk_at_slot2 << (5 * slot_id1)) | (chunk_at_slot1 << (5 * slot_id2)) ;
// }