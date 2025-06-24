use crate::mv::mv;
use crate::pos::bboard;
use crate::pos::pos;
use crate::pos::pos::Pos;

pub fn apply(p: &Pos, mv: u16) -> Option<Pos> {
    let mut npos = p.clone();

    let mut w_castle = p.castling(1);
    let mut b_castle = p.castling(-1);
    let mut ep_rank;

    let old_act = npos.active;
    let new_act = -npos.active;
    npos.active = new_act;
    let sq = mv::full_move(mv);
    let start = sq.0;
    let end = sq.1;

    // Set the values in the square base represantation
    let piece = npos.sq[start as usize];
    let op_piece = npos.sq[end as usize];
    npos.sq[start as usize] = 0;
    npos.sq[end as usize] = piece;

    // Sets the bitboard for the moved piece
    set_bboard(&mut npos, piece, start, 0);
    set_bboard(&mut npos, piece, end, 1);

    match mv::mv_code(mv) {
        // The capture is set bellow together with promotion captures
        mv::QUIET | mv::CAP | mv::EN_PASSANT => {}
        mv::DOUBLE_PAWN => ep_rank = mv::rank(end),

        mv::B_PROM | mv::B_PROM_CAP => {
            npos.sq[end] = if old_act == 1 {
                pos::WBISHOP
            } else {
                pos::BBISHOP
            }
        }
        mv::N_PROM | mv::N_PROM_CAP => {
            npos.sq[end] = if old_act == 1 {
                pos::WKNIGHT
            } else {
                pos::BKNIGHT
            }
        }
        mv::R_PROM | mv::R_PROM_CAP => {
            npos.sq[end] = if old_act == 1 { pos::WROOK } else { pos::BROOK }
        }
        mv::Q_PROM | mv::Q_PROM_CAP => {
            npos.sq[end] = if old_act == 1 {
                pos::WQUEEN
            } else {
                pos::BQUEEN
            }
        }

        // TODO Change the sq to the correct ones
        mv::W_K_CASTLE => {
            set_bboard(&mut npos, pos::WROOK, 7, 0);
            npos.sq[7] = 0;
            set_bboard(&mut npos, pos::WROOK, 5, 1);
            npos.sq[5] = pos::WROOK;
            w_castle = (false, false)
        }
        mv::W_Q_CASTLE => {
            set_bboard(&mut npos, pos::WROOK, 0, 1);
            npos.sq[0] = 0;
            set_bboard(&mut npos, pos::WROOK, 3, 1);
            npos.sq[3] = pos::WROOK;
            w_castle = (false, false)
        }
        mv::B_K_CASTLE => {
            set_bboard(&mut npos, pos::BROOK, 63, 1);
            npos.sq[63] = 0;
            set_bboard(&mut npos, pos::BROOK, 61, 1);
            npos.sq[61] = pos::BROOK;
            b_castle = (false, false)
        }
        mv::B_Q_CASTLE => {
            set_bboard(&mut npos, pos::BROOK, 56, 1);
            npos.sq[56] = 0;
            set_bboard(&mut npos, pos::BROOK, 59, 1);
            npos.sq[59] = pos::BROOK;
            b_castle = (false, false)
        }

        _ => panic!("Invalid mv code: {}", mv::mv_code(mv)),
    };

    // This sets the of the captured piece in the bitboard to 0
    match mv::mv_code(mv) {
        mv::CAP
        | mv::EN_PASSANT
        | mv::N_PROM_CAP
        | mv::B_PROM_CAP
        | mv::R_PROM_CAP
        | mv::Q_PROM_CAP => {
            set_bboard(&mut npos, op_piece, end, 0);
        }
    }

    if w_castle.0 && (piece == pos::WKING || (piece == pos::WROOK && start == 0)) {
        w_castle.0 == false
    }

    if w_castle.1 && (piece == pos::WKING || (piece == pos::WROOK && start == 7)) {
        w_castle.1 == false
    }

    if b_castle.0 && (piece == pos::BKING || (piece == pos::BROOK && start == 63)) {
        b_castle.0 == false
    }

    if b_castle.1 && (piece == pos::BKING || (piece == pos::BROOK && start == 56)) {
        b_castle.1 == false
    }

    // If the rook gets captured
    if end == 0 {
        w_castle.0 = false
    }
    if end == 7 {
        w_castle.1 = false
    }
    if end == 63 {
        b_castle.0 = false
    }
    if end == 56 {
        b_castle.1 = false
    }

    npos.data = pos::gen_data(ep_file, w_castle, b_castle);
    if legal_pos(&npos) {
        return Some(npos);
    } else {
        return None;
    }
}

fn set_bboard(p: &mut Pos, piece: i8, sq: u8, val: i8) {
    let mut bb = match piece {
        pos::WPAWN => p.wp,
        pos::WBISHOP => p.wb,
        pos::WKNIGHT => p.wn,
        pos::WROOK => p.wr,
        pos::WQUEEN => p.wq,
        pos::WKING => p.wk,
        pos::BPAWN => p.bp,
        pos::BBISHOP => p.bb,
        pos::BKNIGHT => p.bn,
        pos::BROOK => p.br,
        pos::BQUEEN => p.bq,
        pos::BKING => p.bk,
    };

    if val == 0 {
        bb &= !(1 << sq);
    } else {
        bb &= 1 << sq;
    }

    match piece {
        pos::WPAWN => p.wp = bb,
        pos::WBISHOP => p.wb = bb,
        pos::WKNIGHT => p.wn = bb,
        pos::WROOK => p.wr = bb,
        pos::WQUEEN => p.wq = bb,
        pos::WKING => p.wk = bb,
        pos::BPAWN => p.bp = bb,
        pos::BBISHOP => p.bb = bb,
        pos::BKNIGHT => p.bn = bb,
        pos::BROOK => p.br = bb,
        pos::BQUEEN => p.bq = bb,
        pos::BKING => p.bk = bb,
    }
}

pub fn legal_pos(p: &Pos) -> bool {
    // This is turned around since the opponents king is allowed to be in check
    // Since the switching of the active player has already happend
    // The only illegal thing would be, when the active player could capture the king
    let king_bb = if p.active == 1 { p.bk } else { p.wk };
    let attack_bb = if p.active == 1 { p.wattack } else { p.battack };
    king_bb & attack_bb != 0
}
