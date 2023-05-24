use std::collections::{HashMap, HashSet};
use lazy_static::lazy_static;


// global variables
// because performance
static NUMS: &str = "0123456789";
static OPERATORS: &str = "+-*%";
static EQUALITY: &str = "=";
static CHARS: &str = "0123456789+-*%=";

// statics
lazy_static! {
    static ref EVAL_MAPPING: HashMap<char, i32> = HashMap::from([
        ('+', 0),
        ('-', 1),
        ('*', 2),
        ('%', 3),
    ]);
}


// algorithms
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


enum Color {
    GREEN,
    YELLOW,
    GREY,
}


struct Guesser {
    searches: i32,
}


impl Guesser {
    fn new() -> Guesser {
        Guesser { searches: 0 }
    }
}