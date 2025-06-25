use crate::util;
/*
    This is a file for tracking and generating different constants.
    Such as different masks for pieces on each square,
    magic bitboard numbers and premasks for the different pieces
*/

// Masks when we want to check for specific file/ rank
// e.g. When cheching if a pawn can queen on the next turn
// RANK[0] corresponds to RANK 1 (not like they are displayed here)
pub const RANK_MASKS: [u64; 8] = [
    0x00000000000000FF,
    0x000000000000FF00,
    0x0000000000FF0000,
    0x00000000FF000000,
    0x000000FF00000000,
    0x0000FF0000000000,
    0x00FF000000000000,
    0xFF00000000000000,
];

pub const FILE_MASKS: [u64; 8] = [
    0x0101010101010101,
    0x0202020202020202,
    0x0404040404040404,
    0x0808080808080808,
    0x1010101010101010,
    0x2020202020202020,
    0x4040404040404040,
    0x8080808080808080,
];

pub static mut BISHOP_PREMASKS: [u64; 64] = [0; 64];
pub static mut ROOK_PREMASKS: [u64; 64] = [0; 64];

pub static mut KNIGHT_MOVEMASKS: [u64; 64] = [0; 64];
pub static mut KING_MOVEMASKS: [u64; 64] = [0; 64];

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

pub fn init_premasks() {}

pub fn init_movemask() {
    // King masks
    let mut sq: i8;
    for sq in 0..63 {
        let mut pos_sq = Vec::new();
        for offset in vec![1, -1, 8, -8, 7, -7, 9, -9] {
            let new_pos = sq + offset;
            if new_pos > 0 && new_pos < 64 && util::util::no_wrap(sq, new_pos) {
                pos_sq.push(new_pos);
            }
        }
        let bb = util::mask::one_at(pos_sq);
        unsafe {
            KING_MOVEMASKS[sq as usize] = bb;
        }
    }
}

pub const ROOK_MAGIC: [u64; 64] = [];
pub const BISHOP_MAGIC: [u64; 64] = [];
