use std::collections::HashSet;
use crate::game::{AllInfo, create_secret, set_colors};


type CreateGuess = fn(&AllInfo, usize) -> String;

/// Test a secret with a given guess function once
pub fn test_once(
    secret: &str,
    create_better_guess: CreateGuess,
) -> (i32, Vec<String>) {
    let mut guessed: Vec<String> = vec![];
    let mut guesses = 0;
    let mut all_info: AllInfo = vec![];

    loop {
        let guess = create_better_guess(&all_info, secret.len());
        println!("{}", guess);

        let colors = set_colors(secret, &guess);
        all_info.push(colors);

        if guess.len() != secret.len() {
            panic!("length error in guess");
        }

        guesses += 1;
        guessed.push(guess.clone());

        if secret == guess {
            break;
        }
    }

    (guesses, guessed)
}


pub fn test(
    difficulty: usize,
    create_better_guess: CreateGuess,
) {
    let mut total = 0;
    let num_secrets = 1000;
    let n = 100 * num_secrets;

    let mut all_guesses: Vec<i32> = vec![];

    // generate secrets
    for _ in 0..num_secrets {
        let secret = create_secret(difficulty);
        // for each secret, test n times//num_secret times
        for _ in 0..(n / num_secrets) {
            let (guesses, _guessed) = test_once(&secret, create_better_guess);
            // add guesses
            all_guesses.push(guesses);
            total += guesses;
        }
    }

    // print data
    println!("max: {}", all_guesses.iter().max().unwrap());
    println!("min: {}", all_guesses.iter().min().unwrap());

    let counts = HashSet::<&i32>::from_iter(all_guesses.iter());
    let mut counts: Vec<&i32> = Vec::from_iter(counts.into_iter());
    counts.sort();

    for count in counts.into_iter() {
        // count the number of guesses = count
        let times = all_guesses.iter()
            .filter(|c| *c == count)
            .count();

        println!("{} = {}", count, times);
    }
    println!("average: {:.2}", (total as f64) / (n as f64));

}
