use crate::eval::eval;
use crate::move_gen;
use std::time;

use crate::move_gen::state;

pub fn get_best_moves(s: &state::State, time: u64) -> (f64, (u8, u8), u8) {
    iter_deepening(s, time)
}

fn iter_deepening(s: &state::State, time: u64) -> (f64, (u8, u8), u8) {
    let dur = time::Duration::from_secs(time);
    let start = time::Instant::now();
    let mut depth = 1;

    let mut res = (0.0, (0, 0));
    loop {
        if time::Instant::now() - start >= dur {
            break;
        }
        res = negamax(s, depth, f64::MIN, f64::MAX);
        depth += 1;
    }
    (res.0, res.1, depth)
}

fn negamax(s: &state::State, depth: u8, mut a: f64, mut b: f64) -> (f64, (u8, u8)) {
    if depth == 0 {
        return (eval::material_eval(s) as f64, (0, 0));
    }

    let moves = move_gen::move_gen::moves(s);
    let mut score = f64::MIN;
    let mut best_move: (u8, u8) = (0, 0);

    for m in moves {
        let outcome = move_gen::outcome::outcome(s, m);
        let res = negamax(&outcome, depth - 1, -b, -a);
        let eval = -res.0;
        if eval > score {
            score = eval;
            if eval > a {
                a = eval;
            }
            best_move = m
        }
        if score >= b {
            return (score, best_move);
        }
    }
    (score, best_move)
}
