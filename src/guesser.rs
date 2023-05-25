use std::collections::{HashMap, HashSet};
use lazy_static::lazy_static;

struct Guesser {
    searches: i32,
}


impl Guesser {
    fn new() -> Guesser {
        Guesser { searches: 0 }
    }
}