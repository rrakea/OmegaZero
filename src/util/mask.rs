pub fn one_at(b: Vec<u8>) -> u64 {
    let mut mask = 0;
    for bit in b {
        mask |= 1 << bit;
    }
    mask
}

pub fn zero_at(b: Vec<u8>) -> u64 {
    let mut mask = u64::MAX;
    for bit in b {
        mask ^= 1 << bit
    }
    mask
}

pub fn one_single(b: u8) -> u64 {
    0 | (1 << b)
}

pub fn zero_single(b: u8) -> u64 {
    u64::MAX ^ (1 << b)
}
