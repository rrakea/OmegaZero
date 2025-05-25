use crate::move_gen::state;

const PAWN_VALUE: i8 = 1;
const KNIGHT_VALUE: i8 = 3;
const BISHOP_VALUE: i8 = 3;
const ROOK_VALUE: i8 = 5;
const QUEEN_VALUE: i8 = 9;
const KING_VALUE: i8 = 0;

pub fn material_eval(s: &state::State) -> i8 {
    let mut eval: i8 = 0;
    let mut self_king = false;
    let mut op_king = false;

    let active = s.active();
    for &p in &s.board {
        match p * active {
            state::PAWN => eval += PAWN_VALUE,
            state::KNIGHT => eval += KNIGHT_VALUE,
            state::BISHOP => eval += BISHOP_VALUE,
            state::ROOK => eval += ROOK_VALUE,
            state::QUEEN => eval += QUEEN_VALUE,
            state::KING => {
                eval += KING_VALUE;
                self_king = true
            }

            state::BPAWN => eval -= PAWN_VALUE,
            state::BKNIGHT => eval -= KNIGHT_VALUE,
            state::BBISHOP => eval -= BISHOP_VALUE,
            state::BROOK => eval -= ROOK_VALUE,
            state::BQUEEN => eval -= QUEEN_VALUE,
            state::BKING => {
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
