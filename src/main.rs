use bevy_game::Guesser;
use clap::{ArgEnum, Parser};

const GAMES: &str = include_str!("../answers.txt");

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, arg_enum)]
    implementation: Implementation,

    #[clap(short, long)]
    max: Option<usize>,
}

#[derive(ArgEnum, Debug, Clone, Copy)]
enum Implementation {
    Naive,
    Allocs,
    Vecrem,
    Once,
    Prune,
}

fn main() {
    let args = Args::parse();

    match args.implementation {
        Implementation::Naive => {
            play(bevy_game::algorithms::Naive::new, args.max);
        }
        Implementation::Allocs => {
            play(bevy_game::algorithms::Allocs::new, args.max);
        }
        Implementation::Vecrem => {
            play(bevy_game::algorithms::Vecrem::new, args.max);
        }
        Implementation::Once => {
            play(bevy_game::algorithms::OnceInit::new, args.max);
        }
        Implementation::Prune => {
            play(bevy_game::algorithms::Prune::new, args.max);
        }
    }
}

fn play<G>(mut mk: impl FnMut() -> G, max: Option<usize>)
where
    G: Guesser,
{
    let w = bevy_game::Wordle::new();
    let mut score = 0;
    let mut games = 0;
    for answer in GAMES.split_whitespace().take(max.unwrap_or(usize::MAX)) {
        let guesser = (mk)();
        if let Some(s) = w.play(answer, guesser) {
            games += 1;
            score += s;
            println!("guessed '{}' in {}", answer, s);
        } else {
            println!("Failed to guess");
        }
    }
    println!("average score: {:.2}", score as f64 / games as f64);
}
