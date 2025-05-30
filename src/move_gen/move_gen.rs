use super::state;
use crate::mv::mv;
use std::iter;

// Initilaizing the arrays so that every function call doesnt have to remake them
static BISHOP_OFFSETS: [i8; 4] = [7, 9, -7, -9];
static ROOK_OFFSETS: [i8; 4] = [1, -1, 8, -8];
static QUEEN_OFFSETS: [i8; 8] = [1, -1, 8, -8, 7, -7, 9, -9];
static KING_OFFSETS: [i8; 8] = [1, -1, 8, -8, 7, -7, 9, -9];
static KNIGHT_OFFSETS: [i8; 8] = [-10, 6, 15, 17, 10, -6, -15, -17];

pub fn mv_gen(s: &state::State, best: &u16, second: &u16) -> impl Iterator<Item = u16> {
    iter::once_with(|| wrapper(*best, *second))
        .chain(iter::once_with(|| promotions(s)))
        .chain(iter::once_with(|| checks(s)))
        .flat_map(|v| v.into_iter())
}

fn wrapper(best: u16, second: u16) -> Vec<u16> {
    vec![best, second]
}

fn checks(s: &state::State) -> Vec<u16> {
    Vec::new()
}

fn promotions(s: &state::State) -> Vec<u16> {
    Vec::new()
}

pub fn moves(s: &state::State) -> Vec<u16> {
    let mut moves: Vec<u16> = Vec::new();

    let active = s.active();

    for (i, p) in s.board.iter().enumerate() {
        match active * *p {
            0 => {}
            state::PAWN => moves.append(&mut can_pawn_move(s, i as u8)),
            state::KNIGHT => moves.append(&mut can_knight_move(s, i as u8)),
            state::BISHOP => moves.append(&mut can_move(s, &BISHOP_OFFSETS, i as u8, true)),
            state::ROOK => moves.append(&mut can_move(s, &ROOK_OFFSETS, i as u8, true)),
            state::QUEEN => moves.append(&mut can_move(s, &QUEEN_OFFSETS, i as u8, true)),
            state::KING => moves.append(&mut can_move(s, &KING_OFFSETS, i as u8, false)),
            _ => {}
        }
    }

    let castle = s.can_castle(active);
    let kingpos: u8 = if active == 1 { 4 } else { 60 };
    let op = -active;

    // Rust formatter grrr
    if castle.0
        && s.board[kingpos as usize + 1] == 0
        && s.board[kingpos as usize + 2] == 0
        && square_attacked(s, kingpos, op)
        && square_attacked(s, kingpos + 1, op)
        && square_attacked(s, kingpos + 2, op)
    {
        moves.push(mv::gen_mv(kingpos, kingpos + 2, false, false, false));
    }

    if castle.1
        && s.board[kingpos as usize - 1] == 0
        && s.board[kingpos as usize - 2] == 0
        && s.board[kingpos as usize - 3] == 0
        && square_attacked(s, kingpos, op)
        && square_attacked(s, kingpos - 1, op)
        && square_attacked(s, kingpos - 2, op)
    {
        moves.push(mv::gen_mv(kingpos, kingpos - 2, false, false, false))
    }

    moves
}

fn can_move(s: &state::State, offset: &[i8], pos: u8, repeat: bool) -> Vec<u16> {
    let mut moves: Vec<u16> = Vec::new();
    let mut iter: u8;
    let active = s.active();

    // ~ Different directions
    for o in offset {
        if repeat {
            iter = 7; // A move can max be 7 squares long
        } else {
            iter = 1;
        }
        let mut new_pos = pos;
        let mut prev_mod: u8 = pos % 8;
        while iter != 0 {
            let p: i32 = new_pos as i32 + *o as i32;
            // Out of bounds checks
            if p < 0 || p > 63 {
                break;
            } else {
                new_pos = p as u8;
            }

            // Left/ right wrapping check
            let new_mod: u8 = new_pos % 8;
            if i32::abs(new_mod as i32 - prev_mod as i32) > 1 {
                break; // A wrap has occured
            }

            // Can capture
            prev_mod = new_pos % 8;

            // Break if piece of same color
            if (s.board[new_pos as usize] * active) > 0 {
                break;
            }

            // Check for capture
            if (s.board[new_pos as usize] * -active) > 0 {
                // Check if the signs are the same
                moves.push((pos, new_pos));
                // Cant move past captured piece
                break;
            }

            moves.push((pos, new_pos));
            iter -= 1;
        }
    }
    moves
}

fn can_knight_move(s: &state::State, pos: u8) -> Vec<u16> {
    let mut moves: Vec<(u8, u8)> = Vec::new();
    for o in KNIGHT_OFFSETS {
        let new_pos = pos as i16 + o as i16;
        // Out of bounds
        if new_pos < 0 || new_pos > 63 {
            continue;
        }
        // Wrapping
        if i16::abs((pos % 8) as i16 - (new_pos % 8)) > 2 {
            continue;
        }
        // Same piece color
        if (s.board[new_pos as usize] * s.active()) > 0 {
            continue;
        }
        moves.push((pos, new_pos as u8));
    }
    moves
}

fn can_pawn_move(s: &state::State, pos: u8) -> Vec<u16> {
    let mut moves: Vec<(u8, u8)> = Vec::new();
    let active = s.active();
    // a) Square ahead is free
    // b) Is on home row -> double pawn move
    // c) Can capture left/ right (no wrapping!)
    // d) Can en passant

    // Ofmg please just cast this yourself compiler
    let one_sq = (pos as i16 + (8 * active) as i16) as usize;
    let two_sq = (pos as i16 + (16 * active) as i16) as usize;
    // These numbers will never be negative in a game

    if s.board[one_sq] == 0 {
        moves.push((pos, one_sq as u8));
        // Double move
        if (8..16).contains(&pos) && active == 1 && s.board[two_sq] == 0 {
            moves.push((pos, pos + 16))
        }
        if (48..56).contains(&pos) && active == -1 && s.board[two_sq] == 0 {
            moves.push((pos, pos - 16))
        }
    }

    // Pawn captures
    let cap_left = (pos as i16 + (7 * active) as i16);
    if s.board[cap_left as usize] * -active > 0 && pos as i16 % 8 - cap_left % 8 == 1 {
        // Signs are not the same && no wrapping occured
        moves.push((pos, cap_left as u8))
    }
    let cap_right = (pos as i16 + (9 * active) as i16);
    if s.board[cap_right as usize] * active > 0 && pos as i16 % 8 - cap_right % 8 == 1 {
        // Signs are not the same && no wrapping
        moves.push((pos, cap_right as u8))
    }

    // en passant
    if s.en_passant == cap_left as u8 && (pos % 8) as i16 - cap_left % 8 == 1 {
        moves.push((pos, s.en_passant))
    }

    if s.en_passant == cap_right as u8 && (pos % 8) as i16 - cap_right % 8 == 1 {
        moves.push((pos, s.en_passant))
    }

    moves
}

fn square_attacked(s: &state::State, square: u8, color: i8) -> bool {
    // The color of the person that is/isnt attacking the square
    for (i, p) in s.board.iter().enumerate() {}
    false
}
