use crate::cube::core::Cube;

impl Cube{
    pub fn display (&self){
        println!("{:064b}", self.edges);
    }


    pub fn corners_slot_from_int(&self, slot: i64) -> String {
        String::from(match slot {
            0 => "UFR",
            1 => "UBR",
            2 => "UBL",
            3 => "UFL",
            4 => "DFR",
            5 => "DBR",
            6 => "DBL",
            7 => "DFL",
            _ => {""}
        })
    }
    pub fn edges_slot_from_int(&self, slot: i64) -> String {
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
    pub fn show_edges_slots(&self) {

        println!("------ EDGES ------");
        for i in 0..12{
            let mask : i64 = 0b11111;
            let chunk =  (self.edges >> (i * 5)) & mask ;
            let slot = chunk >> 1;
            let is_flipped = chunk & 1;

            let color = if slot == i { "\x1b[32m"} else {"\x1b[31m"};

            print!("{} | {}{}[{}] \x1b[0m|",i,color, self.edges_slot_from_int(i) , self.edges_slot_from_int(slot));
            print!(" FlipState : {} |\n", is_flipped);
        }
    }

    pub fn show_corners_slots(&self){
        println!("------ CORNERS ------");
        for i in 0..8 {
            let mask = 0b11111;
            let chunk = (self.corners >> (i * 5)) & mask;
            let slot = chunk >> 2;
            let orientation = chunk & 0b11;

            let color = if slot == i { "\x1b[32m"} else {"\x1b[31m"};
            print!("{} | {}{}[{}] \x1b[0m|",i,color, self.corners_slot_from_int(i) , self.corners_slot_from_int(slot));
            print!(" Orientation : {} |\n", orientation);
        }
    }
}
