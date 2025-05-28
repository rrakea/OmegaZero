use crate::eval::eval;
use crate::move_gen;
use crate::move_gen::state;
use crate::table::table;
use std::sync;
use std::time;

static START: sync::OnceLock<time::Instant> = sync::OnceLock::new();
static ALLOC_TIME: sync::OnceLock<time::Duration> = sync::OnceLock::new();

static TT: sync::OnceLock<table::TT> = sync::OnceLock::new();

// State, time, zobrist key of start pos -> eval, best move, search depth, time taken
fn search(s: &state::State, time: u64, key: u64) -> (f64, u16, u8, u128) {
    START.set(time::Instant::now());
    ALLOC_TIME.set(time::Duration::from_secs(time));

    let mut depth = 1;
    let mut res = (f64::MIN, 0);

    // Iterative deepening
    loop {
        if START.get().unwrap().elapsed() >= *ALLOC_TIME.get().unwrap() {
            break;
        }
        res = negamax(s, depth, f64::MIN, f64::MAX, 0, key);
        depth += 1;
    }

    (
        res.0,
        res.1,
        depth + 1,
        START.get().unwrap().elapsed().as_millis(),
    )
}

// state, depth, alpha, beta, ply from root, prev zobrist key -> eval, best move
fn negamax(s: &state::State, depth: u8, mut a: f64, mut b: f64, ply: u8, key: u64) -> (f64, u16) {
    // Search is done
    if depth == 0 {
        return (eval::material_eval(s) as f64, 0);
    }

    // We are only 1 move away from root && deep enough into the deepening && time ran out
    if ply == 1 && depth > 6 && START.get().unwrap().elapsed() >= *ALLOC_TIME.get().unwrap() {
        return (f64::MIN, 0);
    }

    // Check Transposition table
    let tt_res = TT.get().unwrap().get(key);
    let tt_miss = false;
    let entry = match tt_res {
        Some(res) => res,
        None => {
            tt_miss = true;
            table::Entry::default()
        }
    };

    // Since the 4 most significant bits represent specially interesting moves (tt lookups & captures & checks)
    // The higher a number is the more interesting it is -> we sort the vector to get the highest moves first
    // This will make the moves that are higher up in the board earlier
    // Which is kind of ok, since they are more aggresive
    // -> You maybe would have to flip this for black though
    let moves: Vec<u16> = move_gen::move_gen::moves(s);
    // Sorts in place but does not retain the order of equal elements (we dont care)
    moves.sort_unstable();

    let mut score = f64::MIN;
    let mut best_move = 0;

    // Alpha beta pruning
    for m in moves {
        let outcome = move_gen::outcome::outcome(s, m);
        let next_key = table::next_zobrist(s, key, m);
        let res = negamax(&outcome, depth - 1, -b, -a, ply + 1, next_key);
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
