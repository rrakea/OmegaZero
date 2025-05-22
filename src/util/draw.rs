use crate::util::state;

pub fn draw(s: state::State, eval: f64, time_used: f64, search_depth: u32) {
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
