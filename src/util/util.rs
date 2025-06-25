
pub fn rank(sq: u8) -> u8 {
    sq / 8
}

pub fn file(sq: u8) -> u8 {
    sq % 8
}

// This accomodates for knight moves 
pub fn no_wrap(a: u8, b: u8) -> bool {
    (a as i16 % 8 - b as i16 % 8).abs() <= 2
}

pub fn same_colors(a: i8, b: i8) -> bool {
    a ^ b >= 0
}
pub fn dif_colors(a: i8, b: i8) -> bool {
    a ^ b < 0
}

pub fn is_op_piece(active: i8, p: i8) -> bool {
    !(active * p >= 0)
}

pub fn is_self_piece(active: i8, p: i8) -> bool {
    active * p >= 0
}
