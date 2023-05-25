use std::hint::black_box;
use std::time::Instant;
use crate::game::AllInfo;

mod guesser;
mod game;
mod test;

fn create_better_guess(info: &AllInfo, difficulty: usize) -> String {
    if info.len() <= 1 {
        return "0".repeat(difficulty);
    }

    return "1+1+1=3".to_owned();
}

fn benchmark() {
    for difficulty in 7..16 {
        println!("Difficulty {}", difficulty);
        let start = Instant::now();

        test::test(difficulty, create_better_guess);

        println!("took {:?}\n", start.elapsed());
    }



}



fn main() {
    benchmark();
    // let now = Instant::now();
    //
    // for _ in 0..100000 {
    //     let expression = "12%13%12".to_owned();
    //     let result = guesser::fast_eval(&expression).unwrap_or(-1);
    //     black_box(result);
    // }
    //
    // let elapsed = now.elapsed();
    // println!("took {:.2?}", elapsed);
    // let secret = game::create_secret(10);
    // println!("{}", secret);




}
