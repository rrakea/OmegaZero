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
        res = negascout(s, depth, f64::MIN, f64::MAX, 0, key);
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
fn negascout(s: &state::State, depth: u8, mut a: f64, b: f64, ply: u8, key: u64) -> (f64, u16) {
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
    let mut tt_hit = true;
    let entry = match tt_res {
        Some(res) => res,
        None => {
            tt_hit = false;
            table::Entry::default()
        }
    };

    let mut best_score = f64::MIN;
    let mut best_move = 0;
    let mut principle_variation = true;

    let mut move_gen = move_gen(s);

    move_gen.find(){
        let outcome = move_gen::outcome::outcome(s, crate::mv::mv::full_move(m));
        let next_key = table::next_zobrist(s, key, m);
        let mut res = (0.0, 0);
        if principle_variation {
            principle_variation = false;
            res = negascout(&outcome, depth - 1, -b, -a, ply + 1, next_key);
        } else {
            // Null window search
            res = negascout(&outcome, depth - 1, -a - 1.0, -a, ply + 1, next_key);
            // You have to do this, since you cant do a "-" before the tupel
            let score = -res.0;
            if a < score && score < b {
                // Failed high -> Full re-search
                res = negascout(&outcome, depth - 1, -b, -a, ply + 1, next_key);
            }
        }
        let score = -res.0;
        a = f64::max(a, score);
        if score > best_score {
            best_score = score;
            best_move = m;
        }
        if a >= b {
            break; // Prune :)
        }
    }

    (best_score, best_move)
}
