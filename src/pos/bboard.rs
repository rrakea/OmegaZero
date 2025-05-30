use super::pos;
use std::arch::asm;

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
