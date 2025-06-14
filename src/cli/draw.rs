use crate::mv::mv;
use crate::pos::pos;

pub fn draw(p: &pos::Pos, eval: f64, time_used: u64, search_depth: u8, top_move: u16) {
    let mut line: Vec<&str> = Vec::new();
    let mut board: Vec<String> = Vec::new();

    for (sq, val) in p.sq.iter().enumerate() {
        if sq % 8 == 0 {
            line.push("\n");
            board.push(line.concat());
            line = Vec::new();
            line.push("|");
        }

        match val {
            0 => line.push(" |"),
            &pos::WPAWN => line.push("\u{2659}|"),
            &pos::WKNIGHT => line.push("\u{2658}|"),
            &pos::WBISHOP => line.push("\u{2657}|"),
            &pos::WROOK => line.push("\u{2656}|"),
            &pos::WQUEEN => line.push("\u{2655}|"),
            &pos::WKING => line.push("\u{2654}|"),

            &pos::BPAWN => line.push("\u{265F}|"),
            &pos::BKNIGHT => line.push("\u{265E}|"),
            &pos::BBISHOP => line.push("\u{265D}|"),
            &pos::BROOK => line.push("\u{265C}|"),
            &pos::BQUEEN => line.push("\u{265B}|"),
            &pos::BKING => line.push("\u{265A}|"),
            _ => {}
        }
    }
    line.push("\n");
    board.push(line.concat());
    board.reverse();

    println!("{}", board.concat());
    println!("Move: {}", prittify_move(p, top_move));
    println!("Active Player: {}", p.active);
    println!("Data: {}", p.data);
    println!("Eval: {}", eval);
    println!("Time used: {}s", time_used);
    println!("Search depth: {}", search_depth);
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

pub fn prittify_move(p: &pos::Pos, m: u16) -> String {
    let m = mv::full_move(m);
    let sq = square_name(m.1);
    let code = p.sq[m.1 as usize] * p.active;
    let name = decode(code);
    format!("{}{}", name, sq)
}

pub fn decode(p: i8) -> String {
    let name = match p {
        pos::WPAWN => "",
        pos::WKING => "K",
        pos::WBISHOP => "B",
        pos::WQUEEN => "Q",
        pos::WKNIGHT => "N",
        pos::WROOK => "R",

        pos::BPAWN => "",
        pos::BKING => "K",
        pos::BBISHOP => "B",
        pos::BQUEEN => "Q",
        pos::BKNIGHT => "N",
        pos::BROOK => "R",
        0 => "Empty",
        _ => "x",
    };
    String::from(name)
}
