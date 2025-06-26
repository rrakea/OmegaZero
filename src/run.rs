use crate::cli;
use crate::mv;
use crate::pos;
use crate::table;
use crate::tree_search;

pub fn run() {
    table::table::init_zobrist_keys();
    mv::constants::init_premasks();
    mv::constants::init_piecemask();

    mv::magic::init_magic();

    let mut pos = pos::pos::start_pos();

    loop {
        let res = tree_search::search::search(&pos, 15);
        let eval = res.0;
        let top_move = res.1;
        let depth = res.2;
        let time_taken = res.3;
        pos = mv::mv_apply::apply(&pos, top_move).unwrap();
        cli::draw::draw(&pos, eval, time_taken, depth, top_move);
    }
}
