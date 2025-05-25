use crate::move_gen::state;

pub fn draw(s: &state::State, eval: f64, time_used: f64, search_depth: u32, top_move: (u8, u8)) {
    let mut line: Vec<&str> = Vec::new();
    let mut board: Vec<String> = Vec::new();

    for (sq, p) in s.board.iter().enumerate() {
        if sq % 8 == 0 {
            line.push("\n");
            board.push(line.concat());
            line = Vec::new();
            line.push("|");
        }

        match p {
            0 => line.push(" |"),
            &state::PAWN => line.push("\u{2659}|"),
            &state::KNIGHT => line.push("\u{2658}|"),
            &state::BISHOP => line.push("\u{2657}|"),
            &state::ROOK => line.push("\u{2656}|"),
            &state::QUEEN => line.push("\u{2655}|"),
            &state::KING => line.push("\u{2654}|"),

            &state::BPAWN => line.push("\u{265F}|"),
            &state::BKNIGHT => line.push("\u{265E}|"),
            &state::BBISHOP => line.push("\u{265D}|"),
            &state::BROOK => line.push("\u{265C}|"),
            &state::BQUEEN => line.push("\u{265B}|"),
            &state::BKING => line.push("\u{265A}|"),
            _ => {}
        }
    }
    line.push("\n");
    board.push(line.concat());
    board.reverse();

    println!("{}", board.concat());
    println!("Move: {}", prittify_move(s, top_move));
    println!("Active Player: {}", s.active());
    println!("Data: {}", s.data);
    println!("Eval: {}", eval);
    println!("Time used: {}s", time_used);
    println!("Search depth: {}", search_depth);
    if s.en_passant != 0 {
        println!("Enpassant Square: {}", s.en_passant)
    }
    println!();
    println!();
}

pub fn square_name(sq: u8) -> String {
    let file = sq % 8;
    let rank = sq / 8;
    let filestr = (b'a' + file) as char;
    let rankstr = (b'1' + rank) as char;
    format!("{}{}", filestr, rankstr)
}
pub fn prittify_move(s: &state::State, m: (u8, u8)) -> String {
    if m == (0, 0) {
        return String::from("null");
    }
    let sq = square_name(m.1);
    let code = s.board[m.1 as usize] * s.active();
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

        state::BPAWN => name = "",
        state::BKING => name = "K",
        state::BBISHOP => name = "B",
        state::BQUEEN => name = "Q",
        state::BKNIGHT => name = "N",
        state::BROOK => name = "R",
        0 => name = "Empty",
        _ => name = "x",
    }
    String::from(name)
}
