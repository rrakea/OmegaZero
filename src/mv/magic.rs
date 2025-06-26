use crate::mv;
use crate::pos::pos::Pos;

pub fn init_magics() {}

pub fn queen_mask(sq: u8, p: &Pos) -> u64 {
    rook_mask(sq, p) | bishop_mask(sq, p)

pub fn rook_mask(sq: u8, p: &Pos) -> u64 {
    let full = p.full;
    let premask = unsafe { mv::constants::ROOK_PREMASKS[sq as usize] };
    let magic = ROOK_MAGIC[sq as usize];
    let shift = ROOK_SHIFT[sq as usize];
    let blocker = premask & full;
    let index = (blocker * magic) >> shift;
    let movemask = unsafe { ROOK_MOVEMASK[sq as usize][index as usize] };
    movemask
}

pub fn bishop_mask(sq: u8, p: &Pos) -> u64 {
    let full = p.full;
    let premask = unsafe { mv::constants::BISHOP_PREMASKS[sq as usize] };
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

static mut ROOK_MOVEMASK: [Vec<u64>; 64] = [Vec::new(); 64];
static mut BISHOP_MOVEMASK: [Vec<u64>; 64] = [Vec::new(); 64];
