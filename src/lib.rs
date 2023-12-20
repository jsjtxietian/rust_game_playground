pub mod algorithms;

pub fn play<G: Guesser>(answer: &'static str, mut guesser: G) -> Option<usize> {
    let mut histroy = Vec::new();
    for i in 1..32 {
        let guess = guesser.guess(&histroy);
        if guess == answer {
            return Some(i);
        }
        let correctness = Correctness::compute(answer, &guess);
        histroy.push(Guess {
            word: guess,
            mask: correctness,
        })
    }

    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Correctness {
    // Green
    Correct,
    // Yellow
    Missplaced,
    // Gray
    Wrong,
}

impl Correctness {
    fn compute(answer: &str, guess: &str) -> [Self; 5] {
        assert_eq!(answer.len(), 5);
        assert_eq!(guess.len(), 5);

        let mut c = [Correctness::Wrong; 5];

        for (i,(a,g)) in answer.chars().zip(guess.chars()).enumerate() {
            if a == g {
                c[i] = Correctness::Correct;
            } else if  {

            }
        }
        
        c
    }
}

pub struct Guess {
    pub word: String,
    pub mask: [Correctness; 5],
}
pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}
