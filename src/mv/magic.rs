pub fn queen_mask(sq: u8, active: i8) -> u64 {
    rook_mask(sq, active) | bishop_mask(sq, active)
}

pub fn rook_mask(sq: u8, active: i8) -> u64 {}

pub fn bishop_mask(sq: u8, active: i8) -> u64 {}
