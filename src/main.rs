use std::hint::black_box;
use std::time::Instant;

mod guesser;


fn main() {
    let now = Instant::now();

    for _ in 0..100000 {
        let expression = "12%13%12".to_owned();
        let result = guesser::fast_eval(&expression).unwrap_or(-1);
        black_box(result);
    }

    let elapsed = now.elapsed();
    println!("took {:.2?}", elapsed);
}
