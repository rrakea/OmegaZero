mod engine;
mod util;

const FEN: &str = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1.";

fn main() {
    let start = util::fen::fen_to_board(util::fen::START);
    util::draw::draw(&start, 0.0, 0.0, 8);
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

fn num_positions(p: &util::state::State, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }
    let moves = util::moves::moves(p);
    let mut tally = 0;
    for m in moves {
        let o = util::outcome::outcome(p, m);
        tally += num_positions(&o, depth - 1);
    }
    tally
}
