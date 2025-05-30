use crate::cli;
use crate::move_gen;
use crate::tree_search;

pub fn run() {
    let start = move_gen::fen::fen_to_board(move_gen::fen::START);
    cli::draw::draw(&start, 0.0, 0.0, 0, (0, 0));
    let mut pos = start;
    loop {
        let res = tree_search::search::get_best_moves(&pos, 15);
        let eval = res.0;
        let top_move = res.1;
        pos = move_gen::outcome::outcome(&pos, top_move);
        cli::draw::draw(&pos, eval, 15.0, res.2 as u32, top_move);
    }
}
