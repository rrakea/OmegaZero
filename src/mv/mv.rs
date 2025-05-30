// Ĺibrary for working with moves encoded as u16
// Encoding
// 0000_000000_000000
// The first bit is reserved to encode a transposition table entry suggestion
// The other 3 bits encode: promotion, check, capture (in this order!)
// The next 6 bits encode the square the piece is moved to
// The last 6 bits encode the square the piece came from

pub fn gen_mv(start: u8, end: u8, cap: bool, check: bool, prom: bool) -> u16 {
    let mut res: u16 = 0;
    res &= start as u16;
    res &= (end as u16) << 6;
    res &= (cap as u16) << 12;
    res &= (check as u16) << 13;
    res &= (prom as u16) << 14;
    res
}

pub fn end_sq(m: u16) -> u8 {
    (m & 0b0000_111111_000000 >> 6) as u8
}

pub fn start_sq(m: u16) -> u8 {
    (m & 0b0000_000000_111111) as u8
}

pub fn is_cap(m: u16) -> bool {
    (m & 0b0001_000000_000000) > 0
}

pub fn is_check(m: u16) -> bool {
    (m & 0b0010_000000_000000) > 0
}

pub fn is_prom(m: u16) -> bool {
    (m & 0b0100_000000_000000) > 0
}

pub fn is_in_tt(m: u16) -> bool {
    (m & 0b1000_000000_000000) > 0
}

pub fn full_move(m: u16) -> (u8, u8) {
    (start_sq(m), end_sq(m))
}
