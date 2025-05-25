use super::state;

pub fn outcome(s: &state::State, m: (u8, u8)) -> state::State {
    let mut ns = s.clone();
    // Special cases, where i cant just set the value to 0 and move on
    // a) Casteling - Done
    // b) en passant - Done
    // c) Promoting
    // d) Double pawn move - Done
    // c) King/ Rook move from backrank

    let active = ns.active();
    ns.data ^= 0b1000_0000; // Flip active player

    let ep = ns.en_passant;
    ns.en_passant = 0;

    let piece = ns.board[m.0 as usize];
    ns.board[m.0 as usize] = 0;
    ns.board[m.1 as usize] = piece;

    // En passant occured
    if piece * active == state::PAWN && m.1 == ep {
        // Delete the pawn that was just en passanted
        let pawn_pos = m.1 as i16 - (8 * active) as i16;
        ns.board[pawn_pos as usize] = 0;
    }
    // Set the en_passant value
    if piece * active == state::PAWN && m.1 as i16 - m.0 as i16 == 16 {
        ns.en_passant = m.0 + 8;
    }

    // Castling
    if piece * active == state::KING && m.1 as i16 - m.0 as i16 == 2 {
        let dir: i16 = if m.1 > m.0 { 1 } else { -1 };
        let rookpos = m.1 as usize - dir as usize;
        ns.board[rookpos] = state::ROOK * active;
        // Clear the rook squares
        ns.board[(m.1 as i16 + dir) as usize] = 0;
        ns.board[(m.1 as i16 + dir + dir) as usize] = 0;
    }

    if (piece * active == state::PAWN) && ((0..8).contains(&m.1) || (56..64).contains(&m.1)) {
        ns.board[m.1 as usize] = state::QUEEN * active;
    }

    ns
}
