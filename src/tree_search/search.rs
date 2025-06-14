use crate::eval::eval;
use crate::mv;
use crate::pos::pos;
use crate::table::table;
use once_cell::sync::Lazy;
use std::sync::RwLock;
use std::time;

// This will stop working in 292 billion years :(
static mut START: u64 = 0;
static mut END: u64 = 0;

// This will not stay as a rwlock for long :)
static TT: Lazy<RwLock<table::TT>> = Lazy::new(|| RwLock::new(table::init_transposition_table()));

// State, time, zobrist key of start pos -> eval, best move, search depth, time taken
// Time in milliseconds!!!!!
pub fn search(p: &pos::Pos, time: u64, key: u64) -> (f64, u16, u8, u64) {
    // Safe since none of the threads have started searching yet
    // Wont be mutated till the next move is made
    unsafe {
        START = time::SystemTime::now()
            .duration_since(time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        END = START + time;
    }
    let mut depth = 1;

    // Iterative deepening
    loop {
        if current_time() >= unsafe { END } {
            break;
        }
        negascout(p, depth, f64::MIN, f64::MAX, 0, key);
        depth += 1;
    }

    // Look up the results in the TT table
    let res = TT.read().unwrap().get(key);
    if res.key != key {
        // This should NEVER happen if the hashing is any good
        println!("Well.. fuck. Overwritten the starting position entry")
    }
    (
        res.score as f64,
        res.best,
        depth + 1,
        current_time() - unsafe { START },
    )
}

// state, depth, alpha, beta, ply from root, prev zobrist key -> eval, best move
fn negascout(p: &pos::Pos, depth: u8, mut a: f64, b: f64, ply: u8, key: u64) -> f64 {
    // Search is done
    if depth == 0 {
        return eval::material_eval(p) as f64;
    }

    // Check Transposition table
    let entry = TT.read().unwrap().get(key);

    // Since the search is better than ours will be
    // This also takes care of repetitions and transpositions
    if entry.depth > depth {
        return entry.score as f64;
    }

    // We are only 1 move away from root && deep enough into the deepening && time ran out
    if ply == 1 && depth > 6 && current_time() >= unsafe { END } {
        return entry.score as f64;
    }

    let mut best_score = f64::MIN;
    let mut best_move = 0;

    let mut second_score = f64::MIN;
    let mut second_move = 0;

    // Iterator
    let move_gen = mv::mv_gen::mv_gen(p, &entry.best, &entry.second);

    for (i, m) in move_gen.enumerate() {
        let outcome = mv::mv_apply::apply(p, m);
        let outcome = match outcome {
            Some(o) => o,
            None => continue,
        };
        let next_key = table::next_zobrist(p, key, m);
        let mut score;
        if i < 2 {
            // Transposition table hits
            score = -negascout(&outcome, depth - 1, -b, -a, ply + 1, next_key);
        } else {
            // Null window search
            score = -negascout(&outcome, depth - 1, -a - 1.0, -a, ply + 1, next_key);
            // You have to do this, since you cant do a "-" before the tupel
            if a < score && score < b {
                // Failed high -> Full re-search
                score = -negascout(&outcome, depth - 1, -b, -a, ply + 1, next_key);
            }
        }
        a = f64::max(a, score);
        if score > best_score {
            best_score = score;
            best_move = m;
        } else if score > second_score {
            second_score = score;
            second_move = m;
        }

        if a >= b {
            break; // Prune :)
        }
    }
    // Age and node type not set
    let new_entry = table::Entry {
        key,
        best: best_move,
        second: second_move,
        score: best_score as i8,
        depth,
        node_type: 0,
        age: 0,
    };

    // If our depth is lower we would have quit out before
    {
        TT.write().unwrap().set(new_entry);
    }
    best_score
}

fn current_time() -> u64 {
    time::SystemTime::now()
        .duration_since(time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}
