pub mod cli;
pub mod eval;
pub mod mv;
pub mod pos;
pub mod run;
pub mod table;
pub mod tree_search;
pub mod util;

fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clear Screen
    run::run();
}
