use dwordle::Wordle;

const GAMES: &str = include_str!("../answers.txt");

fn main() {
    for answer in GAMES.split_whitespace() {
        let guesser = dwordle::algorithms::Naive::new();
        let w = Wordle::new();
        w.play(answer, guesser);
    }
}
