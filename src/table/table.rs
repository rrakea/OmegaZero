use crate::pos::pos;
use rand::RngCore;

pub struct TT {
    t: Vec<Entry>,
}

#[derive(Clone, Default)]
pub struct Entry {
    pub key: u64,
    pub best: u16,
    pub second: u16,
    pub score: i8,
    pub depth: u8,
    pub node_type: i8, // -1 -> lower bound; 0 -> exact; 1 -> upper bound
    pub age: u8,
}

const TABLE_SIZE: usize = usize::pow(2, 22);

// Zobrist Keys
pub static mut PAWN: [u64; 64] = [0; 64];
pub static mut KNIGHT: [u64; 64] = [0; 64];
pub static mut BISHOP: [u64; 64] = [0; 64];
pub static mut ROOK: [u64; 64] = [0; 64];
pub static mut QUEEN: [u64; 64] = [0; 64];
pub static mut KING: [u64; 64] = [0; 64];

pub static mut BPAWN: [u64; 64] = [0; 64];
pub static mut BKNIGHT: [u64; 64] = [0; 64];
pub static mut BBISHOP: [u64; 64] = [0; 64];
pub static mut BROOK: [u64; 64] = [0; 64];
pub static mut BQUEEN: [u64; 64] = [0; 64];
pub static mut BKING: [u64; 64] = [0; 64];

pub static mut BLACK: u64 = 0;
pub static mut EN_PASSANT: [u64; 8] = [0; 8];
pub static mut CASTLE: [u64; 4] = [0; 4];

pub fn init_transposition_table() -> TT {
    let table: Vec<Entry> = vec![Entry::default(); TABLE_SIZE];
    TT { t: table }
}

impl TT {
    pub fn get(&self, key: u64) -> Entry {
        let index: usize = key as usize % TABLE_SIZE;
        let e = &self.t[index as usize];
        if e.key == key {
            e.clone()
        } else {
            if e.key != 0 {
                println!("20 Bit Hash Collision!")
            }
            Entry::default()
        }
    }

    pub fn set(&mut self, entry: Entry) {
        let index = entry.key as usize % TABLE_SIZE;
        self.t[index] = entry;
    }
}

pub fn init_zobrist_keys() {
    let mut rng = rand::rng();
    let mut keys = [[0; 64]; 12];
    for i in 0..12 {
        for j in 0..12 * 64 {
            keys[j][i] = rng.next_u64();
        }
    }
    unsafe {
        PAWN = keys[0];
        KNIGHT = keys[1];
        BISHOP = keys[2];
        ROOK = keys[3];
        QUEEN = keys[4];
        KING = keys[5];

        BPAWN = keys[6];
        BKNIGHT = keys[7];
        BBISHOP = keys[8];
        BROOK = keys[9];
        BQUEEN = keys[10];
        BKING = keys[11];
    }

    let mut ep = [0; 8];
    for i in 0..8 {
        ep[i] = rng.next_u64();
    }
    unsafe { EN_PASSANT = ep }

    let mut castle = [0; 4];
    for i in 0..4 {
        castle[i] = rng.next_u64();
    }
    unsafe { CASTLE = castle };

    unsafe { BLACK = rng.next_u64() }
}

pub fn zobrist(p: &pos::Pos) -> u64 {
    let mut keys: Vec<u64> = Vec::new();
    for (i, val) in p.sq.iter().enumerate() {
        match *val {
            pos::WPAWN => keys.push(unsafe { PAWN[i] }),
            pos::WKNIGHT => keys.push(unsafe { KNIGHT[i] }),
            pos::WBISHOP => keys.push(unsafe { BISHOP[i] }),
            pos::WROOK => keys.push(unsafe { ROOK[i] }),
            pos::WQUEEN => keys.push(unsafe { QUEEN[i] }),
            pos::WKING => keys.push(unsafe { KING[i] }),

            pos::BPAWN => keys.push(unsafe { BPAWN[i] }),
            pos::BKNIGHT => keys.push(unsafe { BKNIGHT[i] }),
            pos::BBISHOP => keys.push(unsafe { BBISHOP[i] }),
            pos::BROOK => keys.push(unsafe { BROOK[i] }),
            pos::BQUEEN => keys.push(unsafe { BQUEEN[i] }),
            pos::BKING => keys.push(unsafe { BKING[i] }),
            _ => (),
        }
    }
    let active = p.active;
    if active == -1 {
        keys.push(unsafe { BLACK });
    }

    let wc = p.castling(1);
    let bc = p.castling(-1);
    let ckey = unsafe { CASTLE };
    if wc.0 {
        keys.push(ckey[0]);
    }
    if wc.1 {
        keys.push(ckey[1]);
    }
    if bc.0 {
        keys.push(ckey[2]);
    }
    if bc.1 {
        keys.push(ckey[3]);
    }

    let mut hash = keys[0];
    for i in 1..keys.len() {
        hash ^= keys[i];
    }
    hash
}

pub fn piece_hash(sq: usize, piece: i8) -> u64 {
    match piece {
        pos::WPAWN => return unsafe { PAWN[sq] },
        pos::WKNIGHT => return unsafe { KNIGHT[sq] },
        pos::WBISHOP => return unsafe { BISHOP[sq] },
        pos::WROOK => return unsafe { ROOK[sq] },
        pos::WQUEEN => return unsafe { QUEEN[sq] },
        pos::WKING => return unsafe { KING[sq] },

        pos::BPAWN => return unsafe { BPAWN[sq] },
        pos::BKNIGHT => return unsafe { BKNIGHT[sq] },
        pos::BBISHOP => return unsafe { BBISHOP[sq] },
        pos::BROOK => return unsafe { BROOK[sq] },
        pos::BQUEEN => return unsafe { BQUEEN[sq] },
        pos::BKING => return unsafe { BKING[sq] },
        _ => panic!("Invalid piece code for getting a castle hash: {}", piece),
    }
}

pub fn ep_hash(file: u8) -> u64 {
    unsafe { EN_PASSANT[file as usize] }
}

pub fn color_hash() -> u64 {
    unsafe { BLACK }
}

pub fn castel_hash(active: i8, is_king_side: bool) -> u64 {
    match (active, is_king_side) {
        (1, true) => return unsafe { CASTLE[0] },
        (1, false) => return unsafe { CASTLE[0] },
        (-1, true) => return unsafe { CASTLE[0] },
        (-1, false) => return unsafe { CASTLE[0] },
        _ => panic!("Invalid value in castle_hash: {}, {}", active, is_king_side),
    }
}
