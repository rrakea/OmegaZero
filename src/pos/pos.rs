// Iternal representation of the current position.
// Using a hybrid approach of both bitboards and square centric

// w/b for color & p/n/b/r/q/k for the pieces using the common abbreviations
pub struct Pos {
    // Bitboards
    pub wp: u64,
    pub wn: u64,
    pub wb: u64,
    pub wr: u64,
    pub wq: u64,
    pub wk: u64,

    pub bp: u64,
    pub bn: u64,
    pub bb: u64,
    pub br: u64,
    pub bq: u64,
    pub bk: u64,

    // Attack Tables:
    pub wattack: u64,
    pub battack: u64,

    // Square centric representation
    // Using the consts defined bellow
    // (Negativ for black pieces)
    pub sq: [i8; 64],

    // Extra Data
    // Encoded like this: castling:  b queen, b king, w queen, b king; en_passant file;
    pub data: u8,
    // Active player (1 == w; -1 == b)
    pub active: i8,
}

impl Pos {
    fn en_passant_rank(&self) -> u8 {
        self.data & 0b0000_1111
    }

    // -> kingside, queenside
    fn castling(&self, color: i8) -> (bool, bool) {
        if color == 1 {
            (self.data & 0b0001_0000 > 0, self.data & 0b0010_0000 > 0)
        } else {
            (self.data & 0b0100_0000 > 0, self.data & 0b1000_0000 > 0)
        }
    }
}
