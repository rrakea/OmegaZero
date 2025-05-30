use crate::move_gen;

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
