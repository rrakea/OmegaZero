/*
Functions for working with moves encoded as u16
These encodings are purely usefull for manipulating the bitboards after words

Encoding inspired by Chess Programming Wiki:
*/

pub const QUIET: u8 = 0;
pub const CAP: u8 = 1;
pub const W_K_CASTLE: u8 = 2;
pub const W_Q_CASTLE: u8 = 3;
pub const B_K_CASTLE: u8 = 4;
pub const B_Q_CASTLE: u8 = 5;
pub const DOUBLE_PAWN: u8 = 6;
pub const EN_PASSANT: u8 = 7;
pub const N_PROM: u8 = 8;
pub const B_PROM: u8 = 9;
pub const R_PROM: u8 = 10;
pub const Q_PROM: u8 = 11;
pub const N_PROM_CAP: u8 = 12;
pub const B_PROM_CAP: u8 = 13;
pub const R_PROM_CAP: u8 = 14;
pub const Q_PROM_CAP: u8 = 15;

pub fn gen_mv(start: u8, end: u8, code: u8) -> u16 {
    start as u16 | (end as u16) << 6 | (code as u16) << 12
}

pub fn mv_code(mv: u16) -> u8 {
    (mv >> 12) as u8
}

pub fn is_cap(mv: u16) -> bool {
    match mv_code(mv) {
        1 | 7 | 12 | 13 | 14 | 15 => true,
        _ => false,
    }
}

pub fn is_castle(mv: u16) -> bool {
    match mv_code(mv) {
        2 | 3 | 4 | 5 => true,
        _ => false,
    }
}

pub fn is_prom(mv: u16) -> bool {
    if mv_code(mv) > 7 {
        return true;
    }
    false
}

pub fn end_sq(m: u16) -> u8 {
    (m & 0b0000_111111_000000 >> 6) as u8
}

pub fn start_sq(m: u16) -> u8 {
    (m & 0b0000_000000_111111) as u8
}

pub fn full_move(m: u16) -> (u8, u8) {
    (start_sq(m), end_sq(m))
}
