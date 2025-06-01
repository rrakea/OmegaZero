use crate::cli;
use crate::move_gen;
use crate::table;
use crate::tree_search;

pub fn run() {
    let mut pos = move_gen::fen::fen_to_board(move_gen::fen::START);

    table::table::init_zobrist_keys();
    let key = table::table::zobrist(&pos);

    loop {
        let res = tree_search::search::search(&pos, 15, key);
        let eval = res.0;
        let top_move = res.1;
        let depth = res.2;
        let time_taken = res.3;
        pos = move_gen::outcome::outcome(&pos, top_move);
        cli::draw::draw(&pos, eval, time_taken, depth, top_move);
    }
}
