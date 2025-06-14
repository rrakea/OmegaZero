use crate::mv;
use crate::pos::pos;
use rand::RngCore;
use std::sync::OnceLock;

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
static PAWN: OnceLock<[u64; 64]> = OnceLock::new();
static KNIGHT: OnceLock<[u64; 64]> = OnceLock::new();
static BISHOP: OnceLock<[u64; 64]> = OnceLock::new();
static ROOK: OnceLock<[u64; 64]> = OnceLock::new();
static QUEEN: OnceLock<[u64; 64]> = OnceLock::new();
static KING: OnceLock<[u64; 64]> = OnceLock::new();

static BPAWN: OnceLock<[u64; 64]> = OnceLock::new();
static BKNIGHT: OnceLock<[u64; 64]> = OnceLock::new();
static BBISHOP: OnceLock<[u64; 64]> = OnceLock::new();
static BROOK: OnceLock<[u64; 64]> = OnceLock::new();
static BQUEEN: OnceLock<[u64; 64]> = OnceLock::new();
static BKING: OnceLock<[u64; 64]> = OnceLock::new();

static BLACK: OnceLock<u64> = OnceLock::new();
static EN_PASSANT: OnceLock<[u64; 8]> = OnceLock::new();
static CASTLE: OnceLock<[u64; 4]> = OnceLock::new();

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
    PAWN.set(keys[0]);
    KNIGHT.set(keys[1]);
    BISHOP.set(keys[2]);
    ROOK.set(keys[3]);
    QUEEN.set(keys[4]);
    KING.set(keys[5]);

    BPAWN.set(keys[6]);
    BKNIGHT.set(keys[7]);
    BBISHOP.set(keys[8]);
    BROOK.set(keys[9]);
    BQUEEN.set(keys[10]);
    BKING.set(keys[11]);

    let mut ep = [0; 8];
    for i in 0..8 {
        ep[i] = rng.next_u64();
    }
    EN_PASSANT.set(ep);

    let mut castle = [0; 4];
    for i in 0..4 {
        castle[i] = rng.next_u64();
    }
    CASTLE.set(castle);

    BLACK.set(rng.next_u64());
}

pub fn zobrist(p: &pos::Pos) -> u64 {
    let mut keys: Vec<u64> = Vec::new();
    for (i, val) in p.sq.iter().enumerate() {
        match *val {
            pos::WPAWN => keys.push(PAWN.get().unwrap()[i]),
            pos::WKNIGHT => keys.push(KNIGHT.get().unwrap()[i]),
            pos::WBISHOP => keys.push(BISHOP.get().unwrap()[i]),
            pos::WROOK => keys.push(ROOK.get().unwrap()[i]),
            pos::WQUEEN => keys.push(QUEEN.get().unwrap()[i]),
            pos::WKING => keys.push(KING.get().unwrap()[i]),

            pos::BPAWN => keys.push(BPAWN.get().unwrap()[i]),
            pos::BKNIGHT => keys.push(BKNIGHT.get().unwrap()[i]),
            pos::BBISHOP => keys.push(BBISHOP.get().unwrap()[i]),
            pos::BROOK => keys.push(BROOK.get().unwrap()[i]),
            pos::BQUEEN => keys.push(BQUEEN.get().unwrap()[i]),
            pos::BKING => keys.push(BKING.get().unwrap()[i]),
            _ => (),
        }
    }
    let active = p.active;
    if active == -1 {
        keys.push(*BLACK.get().unwrap());
    }

    let wc = p.castling(1);
    let bc = p.castling(-1);
    let ckey = CASTLE.get().unwrap();
    if wc.0 {
        keys.push(ckey[0])
    }
    if wc.1 {
        keys.push(ckey[1])
    }
    if bc.0 {
        keys.push(ckey[2])
    }
    if bc.1 {
        keys.push(ckey[3])
    }

    let mut hash = keys[0];
    for i in 1..keys.len() {
        hash ^= keys[i];
    }
    hash
}

pub fn next_zobrist(p: &pos::Pos, old_key: u64, mv: u16) -> u64 {
    match mv::mv::mv_code(mv) {
        _ => panic!("Wrong move code {}", mv),
    }
}
