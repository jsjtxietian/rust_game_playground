const GAMES: &str = include_str!("../answers.txt");

fn main() {
    let guesser = bevy_game::algorithms::Naive::new();
    for answer in GAMES.split_whitespace() {
        bevy_game::play(answer, guesser);
    }
}
