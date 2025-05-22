pub const PAWN: i8 = 1;
pub const KNIGHT: i8 = 2;
pub const BISHOP: i8 = 3;
pub const ROOK: i8 = 4;
pub const QUEEN: i8 = 5;
pub const KING: i8 = 6;

pub const BPAWN: i8 = -1;
pub const BKNIGHT: i8 = -2;
pub const BBISHOP: i8 = -3;
pub const BROOK: i8 = -4;
pub const BQUEEN: i8 = -5;
pub const BKING: i8 = -6;

pub struct State {
    pub board: [i8; 64],
    pub data: u8,
    pub en_passant: u8,
}

// White = 1; Black = -1
// board:
// 0 = Empty Square
// The pieces are stored like this:
// 1: pawn, 2: knight, 3:bishop 4: rook, 5: queen, 6: king
// The black pieces are stored as negatives

// data:
// first bit is whos turn it is
// Then 3 emtpy bits->
// Then b. queenside castle, b kingside, w queenside, w kingside

// en_passant:
// Store the square behind the pawn (as one value)

impl State {
    pub fn active(&self) -> i8 {
        if self.data & 128 != 0 {
            -1
        } else {
            1
        }
    }

    pub fn can_castle(&self) -> (bool, bool) {
        let w_kingside = &self.data & 1;
        let w_queenside = &self.data & 2;
        let b_kingside = &self.data & 4;
        let b_queenside = &self.data & 8;
        let active = self.active();

        if active == 1 {
            (w_kingside > 0, w_queenside > 0)
        } else {
            (b_kingside > 0, b_queenside > 0)
        }
    }
}
