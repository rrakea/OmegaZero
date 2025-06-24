use crate::mv::mv;
use crate::pos::pos;
use crate::pos::pos::Pos;
use crate::table::table;
use crate::util;

const BOTTOM_LEFT_SQ: usize = 0;
const BOTTOM_RIGHT_SQ: usize = 7;
const TOP_LEFT_SQ: usize = 56;
const TOP_RIGHT_SQ: usize = 63;

pub fn apply(p: &Pos, mv: u16) -> Option<Pos> {
    let mut npos = p.clone();

    // This is the new zobrist key
    // It will be update through this functions through calls to table::xxx_hash()
    let mut key = p.key;

    let mut w_castle = p.castling(1);
    let mut b_castle = p.castling(-1);
    let mut ep_file = p.en_passant_file();
    // Remove the old ep file from the hash
    key ^= table::ep_hash(ep_file);

    let old_act = npos.active;
    let new_act = -npos.active;
    npos.active = new_act;
    key ^= table::color_hash();
    let sq = mv::full_move(mv);
    let start = sq.0 as usize;
    let end = sq.1 as usize;

    // Unset the castling rights since its easier to unset them once
    // and then later set them again rather than update them everywhere they could change
    if w_castle.0 {
        key ^= table::castel_hash(old_act, true)
    }
    if w_castle.1 {
        key ^= table::castel_hash(old_act, false)
    }
    if b_castle.0 {
        key ^= table::castel_hash(old_act, true)
    }
    if b_castle.1 {
        key ^= table::castel_hash(old_act, false)
    }

    // Set the values in the square base represantation
    let piece = npos.sq[start as usize];
    let op_piece = npos.sq[end as usize];
    npos.sq[start] = 0;
    npos.sq[end] = piece;
    key ^= table::piece_hash(start, piece);
    if op_piece != 0 {
        key ^= table::piece_hash(end, piece);
    }

    // Sets the bitboard for the moved piece
    set_bboard(&mut npos, piece, start, 0);
    set_bboard(&mut npos, piece, end, 1);

    match mv::mv_code(mv) {
        // The capture is set bellow together with promotion captures
        mv::QUIET | mv::CAP | mv::EN_PASSANT => {}
        mv::DOUBLE_PAWN => {
            ep_file = util::util::file(end as u8);
            key ^= table::ep_hash(ep_file);
        }

        mv::B_PROM | mv::B_PROM_CAP => {
            let piece = if old_act == 1 {
                pos::WBISHOP
            } else {
                pos::BBISHOP
            };
            npos.sq[end] = piece;
            key ^= table::piece_hash(end, piece);
        }
        mv::N_PROM | mv::N_PROM_CAP => {
            let piece = if old_act == 1 {
                pos::WKNIGHT
            } else {
                pos::BKNIGHT
            };
            npos.sq[end] = piece;
            key ^= table::piece_hash(end, piece);
        }
        mv::R_PROM | mv::R_PROM_CAP => {
            let piece = if old_act == 1 { pos::WROOK } else { pos::BROOK };
            npos.sq[end] = piece;
            key ^= table::piece_hash(end, piece);
        }
        mv::Q_PROM | mv::Q_PROM_CAP => {
            let piece = if old_act == 1 {
                pos::WQUEEN
            } else {
                pos::BQUEEN
            };
            npos.sq[end] = piece;
            key ^= table::piece_hash(end, piece);
        }

        mv::W_K_CASTLE => {
            set_bboard(&mut npos, pos::WROOK, BOTTOM_RIGHT_SQ, 0);
            npos.sq[BOTTOM_RIGHT_SQ] = 0;
            key ^= table::piece_hash(BOTTOM_RIGHT_SQ, pos::WROOK);

            set_bboard(&mut npos, pos::WROOK, BOTTOM_RIGHT_SQ - 2, 1);
            npos.sq[BOTTOM_RIGHT_SQ - 2] = pos::WROOK;
            key ^= table::piece_hash(BOTTOM_RIGHT_SQ - 2, pos::WROOK);

            w_castle = (false, false);
        }
        mv::W_Q_CASTLE => {
            set_bboard(&mut npos, pos::WROOK, BOTTOM_LEFT_SQ, 1);
            npos.sq[BOTTOM_LEFT_SQ] = 0;
            key ^= table::piece_hash(BOTTOM_LEFT_SQ, pos::WROOK);

            set_bboard(&mut npos, pos::WROOK, BOTTOM_LEFT_SQ + 3, 1);
            npos.sq[BOTTOM_LEFT_SQ + 3] = pos::WROOK;
            key ^= table::piece_hash(BOTTOM_LEFT_SQ + 3, pos::WROOK);

            w_castle = (false, false);
        }
        mv::B_K_CASTLE => {
            set_bboard(&mut npos, pos::BROOK, TOP_RIGHT_SQ, 1);
            npos.sq[TOP_RIGHT_SQ] = 0;
            key ^= table::piece_hash(TOP_RIGHT_SQ, pos::BROOK);

            set_bboard(&mut npos, pos::BROOK, TOP_RIGHT_SQ - 2, 1);
            npos.sq[TOP_RIGHT_SQ - 2] = pos::BROOK;
            key ^= table::piece_hash(TOP_RIGHT_SQ - 2, pos::BROOK);

            b_castle = (false, false);
        }
        mv::B_Q_CASTLE => {
            set_bboard(&mut npos, pos::BROOK, TOP_LEFT_SQ, 1);
            npos.sq[TOP_LEFT_SQ] = 0;
            key ^= table::piece_hash(TOP_LEFT_SQ, pos::BROOK);

            set_bboard(&mut npos, pos::BROOK, TOP_LEFT_SQ + 3, 1);
            npos.sq[TOP_LEFT_SQ + 3] = pos::BROOK;
            key ^= table::piece_hash(TOP_LEFT_SQ + 3, pos::BROOK);

            b_castle = (false, false);
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
        _ => (),
    }

    // This is active player agnostic
    if w_castle.0 {
        // If the king moves || the rook moved from starting square || the rook if captured
        if piece == pos::WKING || (piece == pos::WROOK && start == 0) || end == 0 {
            w_castle.0 = false;
        } else {
            //Castling is still legal
            key ^= table::castel_hash(old_act, true);
        }
    }

    if w_castle.1 {
        if piece == pos::WKING || (piece == pos::WROOK && start == 7) || end == 7 {
            w_castle.1 = false;
        } else {
            key ^= table::castel_hash(old_act, false);
        }
    }

    if b_castle.0 {
        if piece == pos::BKING || (piece == pos::BROOK && start == 63) || end == 63 {
            b_castle.0 = false;
        } else {
            key ^= table::castel_hash(old_act, true);
        }
    }

    if b_castle.1 {
        if piece == pos::BKING || (piece == pos::BROOK && start == 56) || end == 56 {
            b_castle.1 = false;
        } else {
            key ^= table::castel_hash(old_act, false);
        }
    }

    npos.data = pos::gen_data(ep_file, w_castle, b_castle);
    npos.key = key;

    if legal_pos(&npos) {
        return Some(npos);
    } else {
        return None;
    }
}

fn set_bboard(p: &mut Pos, piece: i8, sq: usize, val: i8) {
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
        _ => panic!("Invalid pices code: {}", piece),
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
        _ => panic!("Invalid pices code: {}", piece),
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
