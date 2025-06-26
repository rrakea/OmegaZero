use crate::mv;
use crate::pos::pos::Pos;
use crate::util;

// Rooks always see 14 squares on an emptry board, therefor the amount of possible blocker configurations
// per square are the same regardless of square
const MAX_BLOCKER_ROOK: u64 = u64::pow(2, 14);
const ROOK_OFFSETS: [i8; 4] = [1, -1, 8, -8];

pub fn init_magics() {
    // Rook movemasks
    for sq in 0..64 {
        let magic = ROOK_MAGIC[sq];
        let shift = ROOK_SHIFT[sq];
        let premask = unsafe { mv::constants::ROOK_MASKS[sq as usize] };

        for blocker_index in 0..MAX_BLOCKER_ROOK {
            // Calculate all the possible relevant blocker positions
            let blocker = gen_blockers(premask, blocker_index);

            // Calculate all the possible moves
            let mut pos_mv = Vec::new();
            for offset in ROOK_OFFSETS {
                for (i, sq) in (1..8).enumerate() {
                    let new_pos = sq as i8 + offset * i as i8;
                    if new_pos >= 0
                        || new_pos < 64
                        || util::util::no_wrap(
                            (sq as i8 + offset * (i as i8 - 1)) as u8,
                            new_pos as u8,
                        )
                    {
                        pos_mv.push(new_pos as u8);
                    } else {
                        break;
                    }
                }
            }

            let movemask = util::mask::one_at(pos_mv);

            let index = (movemask * magic) >> shift;

            if index < ROOK_MAX_INDEX as u64 {
                unsafe {
                    ROOK_MOVEMASK[sq][index as usize] = movemask;
                }
            } // If not the
        }
    }

    // Bishop movemasks
}

fn gen_blockers(mask: u64, counter: u64) -> u64 {
    let mut res = 0;
    let mut counter_position = 0;

    for b in 0..64 {
        // If the mask has a 1 at position i
        if (mask >> b) & 1 == 1 {
            // We only need to flip the bit in res if
            // the counter actually has a 1 in that position
            if (counter >> counter_position) & 1 == 1 {
                res |= 1 << b;
            }
            // We have consumed a bit from the counter
            counter_position += 1;
        }
    }
    res
}

pub fn queen_mask(sq: u8, p: &Pos) -> u64 {
    rook_mask(sq, p) | bishop_mask(sq, p)
}

pub fn rook_mask(sq: u8, p: &Pos) -> u64 {
    let full = p.full;
    let premask = unsafe { mv::constants::ROOK_MASKS[sq as usize] };
    let magic = ROOK_MAGIC[sq as usize];
    let shift = ROOK_SHIFT[sq as usize];
    let blocker = premask & full;
    let index = (blocker * magic) >> shift;
    let movemask = unsafe { ROOK_MOVEMASK[sq as usize][index as usize] };
    movemask
}

pub fn bishop_mask(sq: u8, p: &Pos) -> u64 {
    let full = p.full;
    let premask = unsafe { mv::constants::BISHOP_MASKS[sq as usize] };
    let magic = BISHOP_MAGIC[sq as usize];
    let shift = BISHOP_SHIFT[sq as usize];
    let blocker = premask & full;
    let index = (blocker * magic) >> shift;
    let movemask = unsafe { BISHOP_MOVEMASK[sq as usize][index as usize] };
    movemask
}

const ROOK_MAGIC: [u64; 64] = [0; 64];
const ROOK_SHIFT: [u8; 64] = [0; 64];
const BISHOP_MAGIC: [u64; 64] = [0; 64];
const BISHOP_SHIFT: [u8; 64] = [0; 64];

const ROOK_MAX_INDEX: usize = 0;
const BISHOP_MAX_INDEX: usize = 0;

static mut ROOK_MOVEMASK: [[u64; ROOK_MAX_INDEX]; 64] = [[0; ROOK_MAX_INDEX]; 64];
static mut BISHOP_MOVEMASK: [[u64; BISHOP_MAX_INDEX]; 64] = [[0; BISHOP_MAX_INDEX]; 64];
