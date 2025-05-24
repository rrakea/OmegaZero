use crate::util::state;
use core::panic;

pub const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub fn fen_to_board(fen: &str) -> state::State {
    let mut b = [0; 64];

    let mut pos: u8 = 63;

    let mut active_player: i8 = 0;
    let mut en_passant: (u8, u8) = (0, 0);
    let mut w_kingside = false;
    let mut w_queenside = false;
    let mut b_kingside = false;
    let mut b_queenside = false;

    let mut space_count: u8 = 0;

    for p in fen.chars() {
        match space_count {
            0 => {
                match p {
                    '1'..='8' => pos -= p as u8 - '0' as u8 - 1,

                    'r' => b[pos as usize] = -state::ROOK,
                    'n' => b[pos as usize] = -state::KNIGHT,
                    'b' => b[pos as usize] = -state::BISHOP,
                    'q' => b[pos as usize] = -state::QUEEN,
                    'k' => b[pos as usize] = -state::KING,
                    'p' => b[pos as usize] = -state::PAWN,

                    'R' => b[pos as usize] = state::ROOK,
                    'N' => b[pos as usize] = state::KNIGHT,
                    'B' => b[pos as usize] = state::BISHOP,
                    'Q' => b[pos as usize] = state::QUEEN,
                    'K' => b[pos as usize] = state::KING,
                    'P' => b[pos as usize] = state::PAWN,

                    '/' => pos += 1,

                    ' ' => {
                        if pos != 0 {
                            panic!("Invalid FEN! Didnt reach all squares. Pos is {}", pos);
                        } else {
                            space_count += 1;
                        }
                    }

                    _ => panic!("FEN not legal; Char {} found in board data", p),
                }
                print!(" {}", pos);
                if pos != 0 {
                    pos -= 1;
                }
            }
            1 => match p {
                'w' => active_player = 1,
                'b' => active_player = -1,
                ' ' => space_count += 1,
                _ => panic!("FEN not legal; Char {} found in active player data", p),
            },
            2 => match p {
                'K' => w_kingside = true,
                'Q' => w_queenside = true,
                'k' => b_kingside = true,
                'q' => b_queenside = true,
                '-' => {}
                ' ' => space_count += 1,
                _ => panic!("FEN not legal; Char {} found in casteling data", p),
            },
            3 => match p {
                'a'..='h' => en_passant.0 = p as u8 - 'a' as u8,
                '1'..='8' => en_passant.1 = p as u8 - '0' as u8 - 1,
                '-' => break, //{}
                ' ' => break, // space_count += 1,
                _ => panic!("FEN not legal; Char {} found in en passant data", p),
            },
            /* 4 => match p {
                '0'..='9' => {} // Ignore 50 move rule
                ' ' => space_count += 1,
                _ => panic!("FEN not legal; Char {} found in en passant data", p),
            },
            5 => { let buf = 0}
            */
            _ => {}
        }
    }

    let mut data = 0;
    if active_player == 1 {
        data += 128
    }
    if w_kingside {
        data += 1
    }
    if w_queenside {
        data += 2
    }
    if b_kingside {
        data += 4
    }
    if b_queenside {
        data += 8
    }

    println!("{} {}", en_passant.0, en_passant.1);
    let en_passant_comp: u8 = en_passant.1 * 8 + en_passant.0;

    state::State {
        board: b,
        data,
        en_passant: en_passant_comp,
    }
}
