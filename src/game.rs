use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fmt::format;
use std::iter::{Iterator, zip};
use lazy_static::lazy_static;
use rand::distributions::{Distribution, Uniform};

/// The games module contains code needed to run a typical
/// FoCdle game, with error checking and info and such.

// global variables
// because performance
// im sure there is a better way to do this
static NUMS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
static OPERATORS: [char; 4] = ['+', '-', '*', '%'];
static EQUALITY: char = '=';
static CHARS: [char; 15] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '-', '*', '%', '='];



lazy_static! {
    // for the `fast_eval` function
    static ref EVAL_MAPPING: HashMap<char, i32> = HashMap::from([
        ('+', 0),
        ('-', 1),
        ('*', 2),
        ('%', 3),
    ]);
}


/// fast_eval operator algorithm
fn run(left: Option<i32>, right: Option<i32>, op: i32) -> Option<i32> {
    if left.is_none() || right.is_none() {
        return None;
    }

    let left = left.unwrap();
    let right = right.unwrap();
    return match op {
        0 => Some(left + right),
        1 => Some(left - right),
        2 => Some(left * right),
        3 => {
            if right == 0 {
                None
            } else {
                Some(left % right)
            }
        }

        _ => None
    };
}

/// Returns the evaluated option on the valid FoCdle expression
///
/// # Arguments
/// * `expression`, the focdle expression
///
/// # Return
/// An option containing None if the expression is invalid, and the result otherwise
pub fn fast_eval(expression: &str) -> Option<i32> {
    const PREC: [i32; 4] = [0, 0, 1, 1];

    // was the last digit op
    let mut was_op = true;
    // tracking nums
    let mut nums: [i32; 3] = [0, 0, 0];
    // nums encountered
    let mut nums_i = 0;
    // tracking ops
    let mut ops: [i32; 2] = [0, 0];
    // ops encountered
    let mut ops_i = 0;

    let mut i = 0;
    while i < expression.len() {
        // we assume nth char with no outofbounds
        let c: char = expression.chars().nth(i).unwrap();

        // parse integer
        if '0' <= c && c <= '9' {
            if !was_op {
                return None;
            }

            if nums_i >= 3 {
                return None;
            }

            was_op = false;

            // attempt to parse the second digit
            if i + 1 < expression.len() {
                let c1 = expression.chars().nth(i + 1).unwrap();

                if '0' <= c1 && c1 <= '9' {
                    nums[nums_i] = (c.to_digit(10).unwrap() * 10
                        + c1.to_digit(10).unwrap()) as i32;
                    nums_i += 1;
                    i += 2;
                    continue;
                }
            }

            nums[nums_i] = (c.to_digit(10).unwrap()) as i32;
            nums_i += 1;
            i += 1;
            continue;
        }

        if was_op {
            return None;
        }

        if ops_i >= 2 {
            return None;
        }

        was_op = true;
        ops[ops_i] = EVAL_MAPPING[&c];
        ops_i += 1;
        i += 1;
    }

    // if eval second op first
    if PREC[ops[1] as usize] > PREC[ops[0] as usize] {
        return run(
            Some(nums[0]),
            run(
                Some(nums[1]),
                Some(nums[2]),
                ops[1],
            ),
            ops[0],
        );
    }

    // else if eval first op first
    return run(
        run(
            Some(nums[0]),
            Some(nums[1]),
            ops[0],
        ),
        Some(nums[2]),
        ops[1],
    );
}


/// focdle color info colors
pub enum Color {
    GREEN,
    YELLOW,
    GREY,
}

pub struct ColorInfo {
    index: usize,
    chara: char,
    color: Color,
}

pub type AllInfo = Vec<Vec<ColorInfo>>;

#[derive(Default)]
struct InfoLookup {
    correct: HashSet<usize>,
    incorrect: HashSet<usize>,
    min: usize,
    max: usize,
}

pub struct InfoIndex {
    lookup: HashMap<char, InfoLookup>,
}

impl InfoIndex {
    /// Returns the InfoIndex based on the given difficulty
    /// and info 2d array
    fn build(difficulty: usize, info: &AllInfo) -> InfoIndex {
        let mut lookup: HashMap<char, InfoLookup> = HashMap::new();

        // initiate table
        // starting with numbers
        for key in NUMS.iter() {
            lookup.insert(key.clone(), InfoLookup {
                correct: HashSet::new(),
                incorrect: HashSet::new(),
                min: 0,
                max: (if *key != '0' { difficulty - 3 } else { difficulty - 7 }) as usize,
            });
        }

        // then operators
        for key in OPERATORS.iter() {
            lookup.insert(key.clone(), InfoLookup {
                correct: HashSet::new(),
                incorrect: HashSet::new(),
                min: 0,
                max: 2,
            });
        }

        lookup.insert('=', InfoLookup {
            correct: HashSet::new(),
            incorrect: HashSet::new(),
            min: 1,
            max: 1,
        });


        // for each past guess, include color info
        for guess in info {
            let mut character_freqs: HashMap<char, usize> = HashMap::new();
            let mut maxed_characters: HashSet<char> = HashSet::new();

            // add info information
            for info in guess {
                let ColorInfo { index, chara, color } = info;

                match color {
                    Color::GREEN => {
                        lookup.get_mut(chara).unwrap().correct.insert(*index);
                        *character_freqs.entry(*chara).or_insert(0) += 1;
                    }

                    Color::YELLOW => {
                        lookup.get_mut(chara).unwrap().incorrect.insert(*index);
                        *character_freqs.entry(*chara).or_insert(0) += 1;
                    }

                    Color::GREY => {
                        lookup.get_mut(chara).unwrap().incorrect.insert(*index);
                        maxed_characters.insert(chara.clone());
                    }
                }
            }

            // update minmax values
            for key in CHARS.iter() {
                let key_lookup = lookup.get_mut(key).unwrap();

                if key_lookup.min == key_lookup.max {
                    continue;
                }

                key_lookup.min = max(
                    key_lookup.min,
                    *character_freqs.get(key).unwrap_or(&0),
                );

                if maxed_characters.contains(&key) {
                    key_lookup.max = key_lookup.min;
                }
            }
        }

        // improve lookup on operators
        let mut found = false;
        let mut correct_operators = HashSet::new();
        for op in OPERATORS.iter() {
            let lookup_op = &lookup[op];

            if lookup_op.min == 2 {
                correct_operators.clear();
                correct_operators.insert(op);
                found = true;
                break;
            }

            if lookup_op.min == 1 {
                correct_operators.insert(op);
            }
        }

        // if operator found twice
        if found && (correct_operators.len() == 1) {
            for op in OPERATORS.iter() {
                let lookup_op = lookup.get_mut(op).unwrap();

                if !correct_operators.contains(op) {
                    lookup_op.min = 0;
                    lookup_op.max = 0;
                }
            }
        } else if correct_operators.len() == 2 {
            // if two correct operators
            for op in OPERATORS.iter() {
                let lookup_op = lookup.get_mut(op).unwrap();

                if correct_operators.contains(op) {
                    lookup_op.max = 1;
                } else {
                    lookup_op.min = 0;
                    lookup_op.max = 0;
                }
            }
        } else if correct_operators.len() == 1 {
            // if a single correct operator
            for op in OPERATORS.iter() {
                let lookup_op = lookup.get_mut(op).unwrap();

                if !correct_operators.contains(op) {
                    lookup_op.max = 1;
                }
            }
        }

        // constraint digits and max digits
        let mut total_min = 0;
        let mut total_max = 0;
        for (key, lu) in lookup.iter() {
            if NUMS.contains(key) {
                total_min += lu.min;
                total_max += lu.max;
            }
        }

        let max_digits = (difficulty - 3) as usize;
        for (key, lu) in lookup.iter_mut() {
            if NUMS.contains(key) && lu.min < lu.max {
                lu.max = max_digits - (total_min - lu.min);
            }
        }

        InfoIndex {
            lookup
        }
    }
}

/// Returns a character frequency hashmap
fn str_frequency(text: &str, default: &[char]) -> HashMap<char, usize> {
    let mut frequencies = HashMap::new();

    // create default
    for key in default.iter() {
        frequencies.insert(key.clone(), 0);
    }

    for key in text.chars() {
        *(frequencies.get_mut(&key).unwrap()) += 1;
    }

    frequencies
}

/// Tests a `guess` equation against `all_info`, a list of known restrictions,
/// one entry in that list from each previous call to set_colors(). Returns
/// True if that `guess` complies with the collective evidence imposed by
/// `all_info`; returns False if any violation is detected. Does not check the
/// mathematical accuracy of the proposed candidate equation.
pub fn passes_restrictions(
    guess: &str,
    index: &InfoIndex,
) -> bool {
    // create guess character frequencies
    let guess_frequency = str_frequency(guess, &CHARS);

    let mut uniques_upperbound = guess.len();

    for key in CHARS.iter() {
        let lookup = &index.lookup[key];

        for correct in lookup.correct.iter() {
            if guess.chars().nth(*correct).unwrap() == *key {
                return false;
            }
        }

        for incorrect in lookup.incorrect.iter() {
            if guess.chars().nth(*incorrect).unwrap() == *key {
                return false;
            }
        }

        if lookup.max == lookup.min {
            if guess_frequency[key] != lookup.min {
                return false;
            }
        } else {
            if guess_frequency[key] < lookup.min {
                return false;
            }
        }

        // trim duplicates
        uniques_upperbound -= max(
            0,
            lookup.min - 1,
        );
    }

    // check if guess is too unique
    let uniques = HashSet::<char>::from_iter(guess.chars()).len();
    if uniques > uniques_upperbound {
        return false;
    }

    // everything passed
    true
}


/// Returns the list of color information for a given `guess` on the target `secret`
pub fn set_colors(secret: &str, guess: &str) -> Vec<ColorInfo> {
    // create secret character frequency dict
    let mut secret_freq: HashMap<char, i32> = HashMap::new();
    for c in secret.chars() {
        *secret_freq.entry(c).or_insert(0) += 1
    }

    // mark greens
    for (secret_char, guess_char) in zip(secret.chars(), guess.chars()) {
        if guess_char == secret_char {
            *secret_freq.entry(guess_char).or_default() -= 1;
        }
    }

    // fill output
    let mut colors = vec![];
    for i in 0..guess.len() {
        let secret_char = secret.chars().nth(i).unwrap();
        let guess_char = guess.chars().nth(i).unwrap();

        if guess_char == secret_char {
            // add green
            colors.push(ColorInfo {
                index: i,
                chara: guess_char,
                color: Color::GREEN,
            });
        } else if secret_freq.contains_key(&guess_char) && secret_freq[&guess_char] > 0 {
            // add yellow
            *secret_freq.entry(guess_char).or_default() -= 1;
            colors.push(ColorInfo {
                index: i,
                chara: guess_char,
                color: Color::YELLOW,
            });
        } else {
            // add grey
            colors.push(ColorInfo {
                index: i,
                chara: guess_char,
                color: Color::GREY,
            });
        }
    }


    colors
}


/// Returns a random valid focdle expression
fn random_expression() -> String {
    // random number
    let number_dist = Uniform::new(0, 100);
    let mut rng = rand::thread_rng();

    let num1 = number_dist.sample(&mut rng);
    let num2 = number_dist.sample(&mut rng);
    let num3 = number_dist.sample(&mut rng);

    let operator_dist = Uniform::new(0, 4);
    let op1 = &OPERATORS[operator_dist.sample(&mut rng) as usize];
    let op2 = &OPERATORS[operator_dist.sample(&mut rng) as usize];

    format!("{num1}{op1}{num2}{op2}{num3}")
}

/// Returns a valid focdle secret of a given difficulty (7-15)
pub fn create_secret(difficulty: usize) -> String {
    if difficulty < 7 || difficulty > 15 {
        panic!("cannot generate a secret of difficulty not between 7 and 15");
    }

    loop {
        let expression = random_expression();

        let outcome = fast_eval(&expression);
        if outcome.is_none() {
            continue;
        }

        let secret = format!("{}={}", expression, outcome.unwrap());
        if secret.len() != difficulty {
            continue;
        }

        return secret;
    }
}



