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
        match  active * *p {
            0 => {}
            state::PAWN => moves.append(&mut can_move(s, &pawn_offsets, i as u8, false)),
            state::KNIGHT => moves.append(&mut can_move(s, &knight_offsets, i as u8, false)),
            state::BISHOP => moves.append(&mut can_move(s, &bishop_offsets, i as u8, true)),
            state::ROOK => moves.append(&mut can_move(s, &rook_offsets, i as u8, true)),
            state::QUEEN => moves.append(&mut can_move(s, &queen_offsets, i as u8, true)),
            state::KING => moves.append(&mut can_move(s, &king_offsets, i as u8, false)),

            state::BPAWN => moves.append(&mut can_move(s, &bpawn_offsets, i as u8, false)),
            state::BKNIGHT => moves.append(&mut can_move(s, &knight_offsets, i as u8, false)),
            state::BBISHOP => moves.append(&mut can_move(s, &bishop_offsets, i as u8, true)),
            state::BROOK => moves.append(&mut can_move(s, &rook_offsets, i as u8, true)),
            state::BQUEEN => moves.append(&mut can_move(s, &queen_offsets, i as u8, true)),
            state::BKING => moves.append(&mut can_move(s, &king_offsets, i as u8, false)),

            _ => panic!("Wierd value found in board representation: {}", p),
        }
    }
    moves
}

fn in_check() -> bool {
    false
}
fn can_move(s: &state::State, offset: &Vec<i8>, pos: u8, repeat: bool) -> Vec<(u8, u8)> {
    let mut moves: Vec<(u8, u8)> = Vec::new();
    for o in offset {
        let new_pos: i32 = pos as i32 + *o as i32;
        if new_pos < 0 || new_pos > 63 {
            break;
        }
        // Wrapping is net yet accounte for
        moves.push((pos, new_pos as u8));
    }
    moves
}

fn negate_board(b [i8; 64]) -> {
    
}
