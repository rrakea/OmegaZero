use super::pos;

const FULL: u64 = u64::MAX;

fn init_bboards(p: &pos::Pos) {}

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

pub fn get_single(bb: u64) -> u8 {
    bb.trailing_zeros() as u8
}
