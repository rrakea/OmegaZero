pub fn init_magic() {}

pub fn queen_mask(sq: u8, active: i8) -> u64 {
    rook_mask(sq, active) | bishop_mask(sq, active)
}

pub fn rook_mask(sq: u8, active: i8) -> u64 {}

pub fn bishop_mask(sq: u8, active: i8) -> u64 {}

pub const ROOK_MAGIC: [u64; 64] = [];
pub const BISHOP_MAGIC: [u64; 64] = [];

/*
    Psudo Code for generating the moves
    fn rook_move(sq: u64) -> u64 {
        let full_board = get_all(pos);
        let premask = ROOK_PREMASKS[sq];
        let magix = ROOK_MAGIC[sq];
        let shift = ROOK_SHIFT[sq];
        let index = ((full_board & premask) * magic) >> shift;
        return attack_bb[index];
    }
*/

