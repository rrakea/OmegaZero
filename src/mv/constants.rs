use crate::util;
/*
    This is a file for tracking and generating different constants.
    Such as different masks for pieces on each square,
    magic bitboard numbers and masks for the different pieces
*/

pub static mut BISHOP_MASKS: [u64; 64] = [0; 64];
pub static mut ROOK_MASKS: [u64; 64] = [0; 64];
pub static mut KNIGHT_MASKS: [u64; 64] = [0; 64];
pub static mut KING_MASKS: [u64; 64] = [0; 64];

pub fn init_piecemask() {
    let bishop_offsets = vec![7, 9, -7, -9];
    let rook_offsets = vec![1, -1, 8, -8];
    let king_offsets = vec![1, -1, 8, -8, 7, -7, 9, -9];
    let knight_offsets = vec![-10, 6, 15, 17, 10, -6, -15, -17];

    unsafe {
        KING_MASKS = mask_from_offset(&king_offsets, 1);
        BISHOP_MASKS = mask_from_offset(&bishop_offsets, 8);
        ROOK_MASKS = mask_from_offset(&rook_offsets, 8);
        KNIGHT_MASKS = mask_from_offset(&knight_offsets, 1);
    }
}

fn mask_from_offset(offset: &Vec<i8>, iterator: i8) -> [u64; 64] {
    let mut mask = [0; 64];

    for sq in 0..64 {
        let mut pos_sq = Vec::new();
        for o in offset {
            for i in 1..=iterator {
                let new_pos = (sq as i8) + (o * i);
                if new_pos >= 0 && new_pos < 64 && util::util::no_wrap(((sq as i8) + (o * (i -1))) as u8, new_pos as u8) {
                    pos_sq.push(new_pos as u8);
                } else {
                    break;
                }
            }
        }
        mask[sq as usize] = util::mask::one_at(pos_sq);
    }
    mask
}

fn print_masks() {
    println!("KING MASKS: \n");
    for (sq, bb) in unsafe { KING_MASKS }.iter().enumerate() {
        println!("{}", sq);
        util::prittify::pritify_bitboard(*bb);
    }
    println!("BISHOP MASKS: \n");
    for (sq, bb) in unsafe { BISHOP_MASKS}.iter().enumerate() {
        println!("{}", sq);
        util::prittify::pritify_bitboard(*bb);
    }
    println!("ROOK MASKS: \n");
    for (sq, bb) in unsafe { ROOK_MASKS}.iter().enumerate() {
        println!("{}", sq);
        util::prittify::pritify_bitboard(*bb);
    }
    println!("KNIGHT MASKS: \n");
    for (sq, bb) in unsafe { KNIGHT_MASKS }.iter().enumerate() {
        println!("{}", sq);
        util::prittify::pritify_bitboard(*bb);
    }
}

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
