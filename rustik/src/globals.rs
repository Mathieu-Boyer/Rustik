

pub struct Move {
    pub face : usize,
    pub is_clockwise : bool
}

pub const MOVE_LIST : [Move; 12] = [
    Move {face : 0, is_clockwise : true},
    Move {face : 0, is_clockwise : false},

    Move {face : 1, is_clockwise : true},
    Move {face : 1, is_clockwise : false},

    Move {face : 2, is_clockwise : true},
    Move {face : 2, is_clockwise : false},

    Move {face : 3, is_clockwise : true},
    Move {face : 3, is_clockwise : false},

    Move {face : 4, is_clockwise : true},
    Move {face : 4, is_clockwise : false},

    Move {face : 5, is_clockwise : true},
    Move {face : 5, is_clockwise : false},
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
