pub mod cli;
pub mod eval;
pub mod move_gen;
pub mod tree_search;

const FEN: &str = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1.";

fn main() {
    let start = move_gen::fen::fen_to_board(move_gen::fen::START);
    cli::draw::draw(&start, 0.0, 0.0, 8);
    /*
    for m in moves {
        println!("{}", util::draw::prittify_move(&start, m));
        outcomes.push(util::outcome::outcome(&start, m));
    }
    println!("{}", outcomes.len());
    */
    for i in (1..8) {
        println!("{}", num_positions(&start, i));
    }
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
