pub mod benchmark;
pub mod cli;
pub mod eval;
pub mod move_gen;
pub mod mv;
pub mod pos;
pub mod run;
pub mod table;
pub mod tree_search;

fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clear Screen
    run::run();
}
