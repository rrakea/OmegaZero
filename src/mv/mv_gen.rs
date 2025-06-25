use crate::mv::constants;
use crate::mv::magic;
use crate::mv::mv;
use crate::pos::pos::Pos;
use crate::pos::{bboard, pos};
use crate::util::mask;
use crate::util::util;
use std::iter;

/*
    TODO: You can for sure rewrite the pawn moves stuff to also use
    mv_from_movemask()
*/

// Generates an iterator, that lazily generates all the possible moves
// -> When a cutoff is reached the rest of the moves dont get generated at all
// The moves are order such that the most likely good moves are at the top
// e.g. Promotions
pub fn mv_gen(p: &Pos, best: &u16, second: &u16) -> impl Iterator<Item = u16> {
    iter::once_with(|| wrapper(*best, *second))
        .chain(iter::once_with(|| promotions(p)))
        .chain(iter::once_with(|| queen(p)))
        .chain(iter::once_with(|| rook(p)))
        .chain(iter::once_with(|| bishop(p)))
        .chain(iter::once_with(|| pawn_cap(p)))
        .chain(iter::once_with(|| knight(p)))
        .chain(iter::once_with(|| castle(p)))
        .chain(iter::once_with(|| king(p)))
        .chain(iter::once_with(|| pawn_ep(p)))
        .chain(iter::once_with(|| pawn_quiet(p)))
        .chain(iter::once_with(|| pawn_double(p)))
        .flat_map(|v| v.into_iter())
}

fn wrapper(best: u16, second: u16) -> Vec<u16> {
    vec![best, second]
}

fn promotions(p: &Pos) -> Vec<u16> {
    let mut mv = Vec::new();
    let rank = if p.active == 1 { 6 } else { 2 };
    let bb = if p.active == 1 { p.wp } else { p.bp };
    // Only pawns that are on the last rank
    let second_rank = bb & constants::RANK_MASKS[rank];
    if second_rank != 0 {
        let potentials = bboard::get(second_rank);
        for pawn in potentials {
            // The square is empty
            // Multiply with active since black would be -8 offser
            let second_pos: u8 = (pawn as i8 + 8 * p.active) as u8;
            if p.sq[(second_pos) as usize] == 0 {
                mv.push(mv::gen_mv(pawn, second_pos, mv::N_PROM));
                mv.push(mv::gen_mv(pawn, second_pos, mv::B_PROM));
                mv.push(mv::gen_mv(pawn, second_pos, mv::R_PROM));
                mv.push(mv::gen_mv(pawn, second_pos, mv::Q_PROM));
            }
            let cap_left: u8 = (pawn as i8 + 7 * p.active) as u8;
            if util::no_wrap(pawn, cap_left)
                && util::dif_colors(p.sq[cap_left as usize], p.sq[pawn as usize])
            {
                mv.push(mv::gen_mv(pawn, cap_left, mv::N_PROM_CAP));
                mv.push(mv::gen_mv(pawn, cap_left, mv::B_PROM_CAP));
                mv.push(mv::gen_mv(pawn, cap_left, mv::R_PROM_CAP));
                mv.push(mv::gen_mv(pawn, cap_left, mv::Q_PROM_CAP));
            }
            let cap_right = (pawn as i8 + 9 * p.active) as u8;
            if util::no_wrap(pawn, cap_right)
                && util::dif_colors(p.sq[cap_right as usize], p.sq[pawn as usize])
            {
                mv.push(mv::gen_mv(pawn, cap_right, mv::N_PROM_CAP));
                mv.push(mv::gen_mv(pawn, cap_right, mv::B_PROM_CAP));
                mv.push(mv::gen_mv(pawn, cap_right, mv::R_PROM_CAP));
                mv.push(mv::gen_mv(pawn, cap_right, mv::Q_PROM_CAP));
            }
        }
    }
    mv
}

fn queen(p: &Pos) -> Vec<u16> {
    let bb = if p.active == 1 { p.wq } else { p.bq };
    let squares = bboard::get(bb);
    let mut mv = Vec::new();
    for sq in squares {
        let movemask = magic::queen_mask(sq, p.active);
        mv.append(&mut mv_from_movemask(p, movemask, sq));
    }
    mv
}

fn rook(p: &Pos) -> Vec<u16> {
    let bb = if p.active == 1 { p.wn } else { p.bn };
    let squares = bboard::get(bb);
    let mut mv = Vec::new();
    for sq in squares {
        let movemask = magic::rook_mask(sq, p.active);
        mv.append(&mut mv_from_movemask(p, movemask, sq));
    }
    mv
}

fn bishop(p: &Pos) -> Vec<u16> {
    let bb = if p.active == 1 { p.wb } else { p.bb };
    let squares = bboard::get(bb);
    let mut mv = Vec::new();
    for sq in squares {
        let movemask = magic::bishop_mask(sq, p.active);
        mv.append(&mut mv_from_movemask(p, movemask, sq));
    }
    mv
}

fn knight(p: &Pos) -> Vec<u16> {
    let bb = if p.active == 1 { p.wk } else { p.bk };
    // There can only be one king
    let sq = bboard::get_single(bb);
    let mut mv = Vec::new();
    let movemask = unsafe { constants::KNIGHT_MOVEMASKS[sq as usize] };
    mv.append(&mut mv_from_movemask(p, movemask, sq));
    mv
}

fn king(p: &Pos) -> Vec<u16> {
    let bb = if p.active == 1 { p.wn } else { p.bn };
    let squares = bboard::get(bb);
    let mut mv = Vec::new();
    for sq in squares {
        let movemask = unsafe { constants::KING_MOVEMASKS[sq as usize] };
        mv.append(&mut mv_from_movemask(p, movemask, sq));
    }
    mv
}

// Gets a mask of all the possible moves a piece can move from
// its current square -> checks wether the squares are occupied by
// enemy/ owner pieces and generates the proper u16 representation
fn mv_from_movemask(p: &Pos, move_mask: u64, start: u8) -> Vec<u16> {
    let pos_moves = bboard::get(move_mask);
    let mut mv = Vec::new();
    for pos_mv in pos_moves {
        let end_sq_val = p.sq[pos_mv as usize];
        if end_sq_val == 0 {
            mv.push(mv::gen_mv(start, pos_mv, mv::QUIET));
        } else if util::dif_colors(end_sq_val, p.active) {
            mv.push(mv::gen_mv(start, pos_mv, mv::CAP));
        } // You dont do anything if the piece is the same color as you
    }
    mv
}

fn pawn_ep(p: &Pos) -> Vec<u16> {
    let mut mv = Vec::new();
    if p.is_en_passant() {
        let ep_file = p.en_passant_file() as i8;
        let left: i8 = ep_file - 1;
        let right: i8 = ep_file + 1;
        let rank = if p.active == 1 { 5 } else { 4 };
        let pawn_code = if p.active == 1 {
            pos::WPAWN
        } else {
            -pos::WPAWN
        };
        if left != -1 && p.sq[(rank * 8 + left) as usize] == pawn_code {
            mv.push(mv::gen_mv(
                (rank * 8 + left) as u8,
                (rank * 8 + ep_file) as u8,
                mv::EN_PASSANT,
            ));
        }

        if right != 8 && p.sq[(rank * 8 + right) as usize] == pawn_code {
            mv.push(mv::gen_mv(
                (rank * 8 + right) as u8,
                (rank * 8 + ep_file) as u8,
                mv::EN_PASSANT,
            ));
        }
    }

    mv
}

fn castle(p: &Pos) -> Vec<u16> {
    let mut mv = Vec::new();

    let can_castle = p.castling(p.active);
    let king_bb = if p.active == 1 { p.wk } else { p.bk };
    let king_pos = bboard::get_single(king_bb);
    let op_attack = if p.active == 1 { p.battack } else { p.wattack };

    // King side
    let king_cant_be_attacked = mask::one_at(vec![king_pos, king_pos + 1, king_pos + 2]);
    if can_castle.0
        && p.sq[king_pos as usize + 1] == 0
        && p.sq[king_pos as usize + 2] == 0
        && king_cant_be_attacked & op_attack == 0
    {
        let code = if p.active == 1 {
            mv::W_K_CASTLE
        } else {
            mv::B_K_CASTLE
        };
        mv.push(mv::gen_mv(king_pos, king_pos + 2, code))
    }

    // Queen side
    let queen_cant_be_attacked = mask::one_at(vec![king_pos, king_pos - 1, king_pos - 2]);
    if can_castle.1
        && p.sq[king_pos as usize - 1] == 0
        && p.sq[king_pos as usize - 2] == 0
        && p.sq[king_pos as usize - 3] == 0
        && queen_cant_be_attacked & op_attack == 0
    {
        let code = if p.active == 1 {
            mv::W_Q_CASTLE
        } else {
            mv::B_Q_CASTLE
        };
        mv.push(mv::gen_mv(king_pos, king_pos - 2, code));
    }
    mv
}

fn pawn_quiet(p: &Pos) -> Vec<u16> {
    let mut mv = Vec::new();
    let possible_positions: u64;
    if p.active == 1 {
        // The pawns cant stand on the last or first rank (0/7)
        // Rank 6 is covered by the promotion function
        possible_positions = constants::RANK_MASKS[1]
            | constants::RANK_MASKS[2]
            | constants::RANK_MASKS[3]
            | constants::RANK_MASKS[4]
            | constants::RANK_MASKS[5];
    } else {
        possible_positions = constants::RANK_MASKS[6]
            | constants::RANK_MASKS[5]
            | constants::RANK_MASKS[4]
            | constants::RANK_MASKS[3]
            | constants::RANK_MASKS[2];
    }
    let bb = if p.active == 1 { p.wp } else { p.bp };
    let pawns = bboard::get(possible_positions ^ bb);
    let offset = if p.active == 1 { 8 } else { -8 };
    for pawn in pawns {
        let second_pos = (pawn as i8 + offset) as u8;
        if p.sq[second_pos as usize] == 0 {
            mv.push(mv::gen_mv(pawn, second_pos, mv::QUIET));
        }
    }
    mv
}

fn pawn_double(p: &Pos) -> Vec<u16> {
    let mut mv = Vec::new();

    let bb = if p.active == 1 { p.wp } else { p.bp };
    let rank = if p.active == 1 { 2 } else { 6 };
    let second_rank = bb ^ constants::RANK_MASKS[rank];

    if second_rank != 0 {
        for pawn in bboard::get(second_rank) {
            let one_move = pawn as i8 + 8 * p.active;
            let two_move = pawn as i8 + 16 * p.active;

            if p.sq[one_move as usize] == 0 && p.sq[two_move as usize] == 0 {
                mv.push(mv::gen_mv(pawn, two_move as u8, mv::DOUBLE_PAWN));
            }
        }
    }

    mv
}

fn pawn_cap(p: &Pos) -> Vec<u16> {
    Vec::new()
}
