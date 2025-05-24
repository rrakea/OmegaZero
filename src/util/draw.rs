use super::state;

pub fn draw(s: &state::State, eval: f64, time_used: f64, search_depth: u32) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clear Screen

    for (sq, p) in s.board.iter().enumerate() {
        if sq % 8 == 0 {
            println!();
            print!("|")
        }

        match p {
            0 => print!(" |"),
            &state::PAWN => print!("\u{2659}|"),
            &state::KNIGHT => print!("\u{2658}|"),
            &state::BISHOP => print!("\u{2657}|"),
            &state::ROOK => print!("\u{2656}|"),
            &state::QUEEN => print!("\u{2655}|"),
            &state::KING => print!("\u{2654}|"),

            &state::BPAWN => print!("\u{265F}|"),
            &state::BKNIGHT => print!("\u{265E}|"),
            &state::BBISHOP => print!("\u{265D}|"),
            &state::BROOK => print!("\u{265C}|"),
            &state::BQUEEN => print!("\u{265B}|"),
            &state::BKING => print!("\u{265A}|"),
            _ => {}
        };
    }

    println!();
    println!();
    println!();
    println!("Active Player: {}", s.active());
    println!("Data: {}", s.data);
    println!("Eval: {}", eval);
    println!("Time used: {}s", time_used);
    println!("Search depth: {}", search_depth);
    if s.en_passant != 0 {
        println!("Enpassant Square: {}", s.en_passant)
    }
}

pub fn square_name(sq: u8) -> String {
    let file = sq % 8;
    let rank = sq / 8;
    let filestr = (b'a' + file) as char;
    let rankstr = (b'1' + rank) as char;
    format!("{}{}", filestr, rankstr)
}
pub fn prittify_move(s: &state::State, m: (u8, u8)) -> String {
    let sq = square_name(m.1);
    let code = s.board[m.0 as usize] * s.active();
    let name = decode(code);
    format!("{}{}", name, sq)
}

pub fn decode(p: i8) -> String {
    let mut name: &str;
    match p {
        state::PAWN => name = "",
        state::KING => name = "K",
        state::BISHOP => name = "B",
        state::QUEEN => name = "Q",
        state::KNIGHT => name = "N",
        state::ROOK => name = "R",
        0 => name = "Empty",
        _ => name = "x",
    }
    String::from(name)
}
