const GAMES: &str = include_str!("../answers.txt");

fn main() {
    let w = bevy_game::Wordle::new();
    for answer in GAMES.split_whitespace() {
        let guesser = bevy_game::algorithms::Naive::new();
        if let Some(score) = w.play(answer, guesser) {
            println!("guessed '{}' in {}", answer, score);
        } else {
            println!("Failed to guess");
        }
    }
}
