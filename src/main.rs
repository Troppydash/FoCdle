use std::env;
use std::time::Instant;
use crate::game::{AllInfo};

mod guesser;
mod game;
mod test;

fn create_better_guess(info: &AllInfo, difficulty: usize) -> String {
    let mut guesser = guesser::Guesser::new(difficulty, info);
    guesser.create_guess()
}

fn benchmark() {
    for difficulty in 7..16 {
        println!("Difficulty {}", difficulty);
        let start = Instant::now();

        test::test(difficulty, create_better_guess);

        println!("took {:?} (for 1000 secrets, 100x each)\n", start.elapsed());
    }
}



fn main() {
    // benchmark();
    let args: Vec<String> = env::args().collect();
    for arg in args[1..].iter() {
        println!("secret {}:", arg);
        test::test_once(arg, create_better_guess);
        println!();
    }
}
