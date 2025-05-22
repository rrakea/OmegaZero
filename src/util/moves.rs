use super::state;
use core::panic;

pub fn moves() {}

fn potential_moves(s: &state::State) -> Vec<(u8, u8)> {
    let mut moves: Vec<(u8, u8)> = Vec::new();
    let active = s.active();

    let pawn_offsets = vec![8];
    let knight_offsets = vec![-10, 6, 15, 17, 10, -1, -15, -17];
    let bishop_offsets = vec![7, 9, -7, -9];
    let rook_offsets = vec![1, -1, 8, -8];
    let queen_offsets = vec![1, -1, 8, -8, 7, -7, 9, -9];
    let king_offsets = vec![1, -1, 8, -8, 7, -7, 9, -9];

    for (i, p) in s.board.iter().enumerate() {
        match active * *p {
            0 => {}
            state::PAWN => moves.append(&mut can_move(s, &pawn_offsets, i as u8, false)),
            state::KNIGHT => moves.append(&mut can_move(s, &knight_offsets, i as u8, false)),
            state::BISHOP => moves.append(&mut can_move(s, &bishop_offsets, i as u8, true)),
            state::ROOK => moves.append(&mut can_move(s, &rook_offsets, i as u8, true)),
            state::QUEEN => moves.append(&mut can_move(s, &queen_offsets, i as u8, true)),
            state::KING => moves.append(&mut can_move(s, &king_offsets, i as u8, false)),
            _ => {}
        }
    }
    // Double Pawn Moves
    // En passant
    // Casteling
    let castle = s.can_castle();

    moves
}

fn in_check() -> bool {
    false
}
fn can_move(s: &state::State, offset: &Vec<i8>, pos: u8, repeat: bool) -> Vec<(u8, u8)> {
    let mut moves: Vec<(u8, u8)> = Vec::new();
    let mut iter: u8;
    let active = s.active();

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
            if p < 0 || p > 63 {
                break;
            } else {
                new_pos = p as u8;
            }

            let new_mod: u8 = new_pos % 8;
            if i32::abs((new_mod - prev_mod) as i32) > 2 {
                // >2 specifically only for the knight
                break; // A wrap has occured
            }
            prev_mod = new_pos % 8;
            if (s.board[new_pos as usize] ^ active) >= 0 {
                // Check if the signs are the same (XOR)
                moves.push((pos, new_pos));
                break;
            }
            // Wrapping is net yet accounte for
            moves.push((pos, new_pos));
            iter -= 1;
        }
    }
    moves
}
