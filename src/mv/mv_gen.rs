use crate::mv::mv;
use crate::pos::bboard;
use crate::pos::pos::Pos;
use std::iter;

/*
    Order:
    Promotions
    Checks
    En passant (Since en passant are time sensitive)
    Captures
    Castles
    Quiet piece moces
    Quiet pawn moves
    Double pawn moves
*/

// RANK[0] corresponds to RANK 1 (not like they are displayed here)
const RANK_MASKS: [u64; 8] = [
    0x00000000000000FF,
    0x000000000000FF00,
    0x0000000000FF0000,
    0x00000000FF000000,
    0x000000FF00000000,
    0x0000FF0000000000,
    0x00FF000000000000,
    0xFF00000000000000,
];

const FILE_MASKS: [u64; 8] = [
    0x0101010101010101,
    0x0202020202020202,
    0x0404040404040404,
    0x0808080808080808,
    0x1010101010101010,
    0x2020202020202020,
    0x4040404040404040,
    0x8080808080808080,
];

pub fn mv_gen(p: &Pos, best: &u16, second: &u16) -> impl Iterator<Item = u16> {
    iter::once_with(|| wrapper(*best, *second))
        .chain(iter::once_with(|| promotions(p)))
        .chain(iter::once_with(|| checks(p)))
        .chain(iter::once_with(|| en_passant(p)))
        .chain(iter::once_with(|| caps(p)))
        .chain(iter::once_with(|| castle(p)))
        .chain(iter::once_with(|| quiet_piece(p)))
        .chain(iter::once_with(|| quiet_pawn(p)))
        .chain(iter::once_with(|| double_pawn(p)))
        .flat_map(|v| v.into_iter())
}

fn wrapper(best: u16, second: u16) -> Vec<u16> {
    vec![best, second]
}

fn attack_tables(p: &Pos) -> (u64, u64) {}

fn promotions(p: &Pos) -> Vec<u16> {
    let mut mv = Vec::new();
    let rank = if p.active == 1 { 6 } else { 2 };
    let bb = if p.active == 1 { p.wp } else { p.bp };
    // Only pawns that are on the last rank
    let second_rank = bb ^ RANK_MASKS[rank];
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
            if no_wrap(pawn, cap_left)
                && dif_colors(
                    p.sq[(pawn as i8 + 7 * p.active) as usize],
                    p.sq[pawn as usize],
                )
            {
                mv.push(mv::gen_mv(pawn, cap_left, mv::N_PROM_CAP));
                mv.push(mv::gen_mv(pawn, cap_left, mv::B_PROM_CAP));
                mv.push(mv::gen_mv(pawn, cap_left, mv::R_PROM_CAP));
                mv.push(mv::gen_mv(pawn, cap_left, mv::Q_PROM_CAP));
            }
            let cap_right = (pawn as i8 + 9 * p.active) as u8;
            if no_wrap(pawn, cap_right)
                && dif_colors(
                    p.sq[(pawn as i8 + 7 * p.active) as usize],
                    p.sq[pawn as usize],
                )
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

fn checks(p: &Pos) -> Vec<u16> {}

fn en_passant(p: &Pos) -> Vec<u16> {}

fn caps(p: &Pos) -> Vec<u16> {}

fn castle(p: &Pos) -> Vec<u16> {
    let mut mv = Vec::new();

    let can_castle = p.castling(p.active);
    let king_bb = if p.active == 1 { p.wk } else { p.bk };
    let king_pos = bboard::get_single(king_bb);
    let op_attack = if p.active == 1 { p.battack } else { p.wattack };

    // King side
    let king_cant_be_attacked = bit_mask(vec![king_pos, king_pos + 1, king_pos + 2]);
    if can_castle.0
        && p.sq[king_pos as usize + 1] == 0
        && p.sq[king_pos as usize + 2] == 0
        && king_cant_be_attacked & op_attack == 0
    {
        mv.push(mv::gen_mv(king_pos, king_pos + 2, 2))
    }

    // Queen side
    let queen_cant_be_attacked = bit_mask(vec![king_pos, king_pos - 1, king_pos - 2]);
    if can_castle.1
        && p.sq[king_pos as usize - 1] == 0
        && p.sq[king_pos as usize - 2] == 0
        && p.sq[king_pos as usize - 3] == 0
        && queen_cant_be_attacked & op_attack == 0
    {
        mv.push(mv::gen_mv(king_pos, king_pos - 2, mv::K_CASTLE));
    }
    mv
}

fn quiet_piece(p: &Pos) -> Vec<u16> {}

fn quiet_pawn(p: &Pos) -> Vec<u16> {}

fn double_pawn(p: &Pos) -> Vec<u16> {
    let mut mv = Vec::new();

    let bb = if p.active == 1 { p.wp } else { p.bp };
    let rank = if p.active == 1 { 2 } else { 6 };
    let second_rank = bb ^ RANK_MASKS[rank];

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

fn bit_mask(b: Vec<u8>) -> u64 {
    let mut res = 0;
    for bit in b {
        res |= 1 << bit;
    }
    res
}

fn no_wrap(a: u8, b: u8) -> bool {
    (a % 8 - b % 8) > 1
}

fn bit_mask_single(b: u8) -> u64 {
    1 << b
}

fn dif_colors(a: i8, b: i8) -> bool {
    !(a ^ b >= 0)
}

fn same_colors(a: i8, b: i8) -> bool {
    a ^ b >= 0
}

fn op_piece(active: i8, p: i8) -> bool {
    !(active * p >= 0)
}

fn self_piece(active: i8, p: i8) -> bool {
    active * p >= 0
}
