mod engine;
mod util;

const FEN: &str = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1.";

fn main() {
    let start = util::fen::fen_to_board(util::fen::START);
    util::draw::draw(start, 0.0, 0.0, 20)
}
