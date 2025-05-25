pub mod cli;
pub mod eval;
pub mod move_gen;
pub mod tree_search;

const FEN: &str = "3rQr1k/p1p3pp/1b6/8/1P6/2q2N2/5PPP/R3R1K1 w - - 2 26";

fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clear Screen
    let start = move_gen::fen::fen_to_board(FEN);
    cli::draw::draw(&start, 0.0, 0.0, 0, (0, 0));
    let mut pos = start;
    for i in 1..8 {
        let res = tree_search::search::get_best_moves(&pos, 15);
        let eval = res.0;
        let top_move = res.1;
        pos = move_gen::outcome::outcome(&pos, top_move);
        cli::draw::draw(&pos, eval, 15.0, res.2 as u32, top_move);
    }
    /*
    for m in moves {
        println!("{}", util::draw::prittify_move(&start, m));
        outcomes.push(util::outcome::outcome(&start, m));
    }
    println!("{}", outcomes.len());
    */
    /*
    for i in (1..8) {
        println!("{}", num_positions(&start, i));
    }
    */
}

fn num_positions(p: &move_gen::state::State, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }
    let moves = move_gen::move_gen::moves(p);
    let mut tally = 0;
    for m in moves {
        let o = move_gen::outcome::outcome(p, m);
        tally += num_positions(&o, depth - 1);
    }
    tally
}
