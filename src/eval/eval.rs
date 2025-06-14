use crate::pos::pos;

const PAWN_VALUE: i8 = 1;
const KNIGHT_VALUE: i8 = 3;
const BISHOP_VALUE: i8 = 3;
const ROOK_VALUE: i8 = 5;
const QUEEN_VALUE: i8 = 9;
const KING_VALUE: i8 = 0;

pub fn material_eval(p: &pos::Pos) -> i8 {
    let mut eval: i8 = 0;
    let mut self_king = false;
    let mut op_king = false;

    let active = p.active;
    for &p in &p.sq {
        match p * active {
            pos::WPAWN => eval += PAWN_VALUE,
            pos::WKNIGHT => eval += KNIGHT_VALUE,
            pos::WBISHOP => eval += BISHOP_VALUE,
            pos::WROOK => eval += ROOK_VALUE,
            pos::WQUEEN => eval += QUEEN_VALUE,
            pos::WKING => {
                eval += KING_VALUE;
                self_king = true
            }

            pos::BPAWN => eval -= PAWN_VALUE,
            pos::BKNIGHT => eval -= KNIGHT_VALUE,
            pos::BBISHOP => eval -= BISHOP_VALUE,
            pos::BROOK => eval -= ROOK_VALUE,
            pos::BQUEEN => eval -= QUEEN_VALUE,
            pos::BKING => {
                eval -= KING_VALUE;
                op_king = true
            }
            _ => {}
        }
    }
    if !self_king {
        return i8::MIN;
    }
    if !op_king {
        return i8::MAX;
    }
    eval
}
