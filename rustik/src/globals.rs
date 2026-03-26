#[derive(Clone)]
pub struct Move {
    pub face : usize,
    pub is_clockwise : bool,
    pub name : &'static str,
    pub num : i8
}

pub const MOVE_LIST : [Move; 18] = [
    Move {face : 0, is_clockwise : true, name : "U", num : 1},
    Move {face : 0, is_clockwise : false, name : "U'", num : 1},
    Move {face : 0, is_clockwise : true, name : "U2", num : 2},

    Move {face : 1, is_clockwise : true, name : "D", num : 1},
    Move {face : 1, is_clockwise : false , name : "D'", num : 1},
    Move {face : 1, is_clockwise : true, name : "D2", num : 2},
    

    Move {face : 2, is_clockwise : true , name : "F", num : 1},
    Move {face : 2, is_clockwise : false , name : "F'", num : 1},
    Move {face : 2, is_clockwise : true, name : "F2", num : 2},
    

    Move {face : 3, is_clockwise : true, name : "B", num : 1},
    Move {face : 3, is_clockwise : false, name : "B'", num : 1},
    Move {face : 3, is_clockwise : true, name : "B2", num : 2},
    

    Move {face : 4, is_clockwise : true , name : "R", num : 1},
    Move {face : 4, is_clockwise : false, name : "R'", num : 1},
    Move {face : 4, is_clockwise : true, name : "R2", num : 2},
    

    Move {face : 5, is_clockwise : true , name : "L", num : 1},
    Move {face : 5, is_clockwise : false, name : "L'", num : 1},
    Move {face : 5, is_clockwise : true, name : "L2", num : 2},
    
];

pub const AFFECTED_EDGE_SLOTS : [[i64; 4]; 6] = [
 [0, 3, 2 , 1],    // U
 [8, 9, 10, 11],  // D
 [0, 4, 8, 7],    // F
 [2, 6, 10, 5],   // B
 [1, 5, 9, 4],    // R
 [3, 7, 11, 6],   // L
];


pub const AFFECTED_CORNERS_SLOTS : [[i64; 4]; 6] = [
  [3, 2, 1, 0],    // U
  [7, 4 , 5 , 6],  // D
  [3, 0 , 4 , 7],  // F
  [1, 2 , 6, 5],   // B
  [0, 1, 5 , 4],   // R
  [3, 7 , 6 , 2],  // L
];


pub const FACES : [usize; 6] = [
 0, 1, 2, 3, 4, 5
];

pub const OPPOSITE_FACE : [usize; 6] = [
  1, 0, 3, 2, 5, 4
];
