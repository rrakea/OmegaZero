use super::pos;
use std::arch::asm;

const FULL: u64 = u64::MAX;

fn init_bboards(p: &pos::Pos) {}

fn population_count(bb: u64) -> u64 {
    // Uses inline assembly to get the amount of bits flipped in a u64 efficiently
    if std::is_x86_feature_detected!("popcnt") {
        let mut res: u64 = 0;
        unsafe { asm!("popcnt {1} {0}", in(reg) bb, out(reg) res) }
        res
    } else {
        panic!("POPCNT not supported :(")
    }
}

pub fn empty(bb: u64) -> bool {
    bb == 0
}

pub fn bb_all(p: &pos::Pos) -> u64 {
    p.wp | p.wn | p.wb | p.wr | p.wq | p.wk | p.bp | p.bn | p.bb | p.br | p.bq | p.bk
}

pub fn bb_w(p: &pos::Pos) -> u64 {
    p.wp | p.wn | p.wb | p.wr | p.wq | p.wk
}

pub fn bb_b(p: &pos::Pos) -> u64 {
    p.bp | p.bn | p.bb | p.br | p.bq | p.bk
}

pub fn none(p: &pos::Pos) -> u64 {
    bb_all(p) ^ FULL
}

pub fn get(mut bb: u64) -> Vec<u8> {
    let mut mv: Vec<u8> = Vec::new();
    let mut lsb;
    while bb != 0 {
        lsb = bb.trailing_zeros();
        mv.push(lsb as u8);
        bb &= bb - 1;
    }
    mv
}

// Is there a better way to do this?
pub fn get_single(mut bb: u64) -> u8 {
    get(bb)[0]
}
