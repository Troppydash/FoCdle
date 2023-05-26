use std::cmp::{min};
use std::collections::{HashMap, HashSet};
use lazy_static::lazy_static;
use rand::{Rng};
use crate::game::{AllInfo, CHARS, fast_eval, InfoIndex, InfoLookup, OPERATORS, passes_restrictions};
use crate::game::NUMS;

/// the initial optimal guesses
static OPTIMAL_GUESSES: [&str; 9] = [
    "1+-*%54",
    "1+-*%=54",
    "1+-*%==44",
    "1+-*%4==15",
    "1+-*%4==165",
    "1+-*%45=1761",
    "1+-3*%5=17611",
    "12+-4*%6187111",
    "12+-4*%61221111"
];

/// returns the loaded json frequency, included at compile-time for performance
fn load_frequency() -> serde_json::Value {
    let text = include_str!("resources/freq2.json");
    let value = serde_json::from_str(text)
        .expect("Cannot parse the content in 'freq2.json'!");

    value
}

lazy_static! {
    static ref FREQUENCY: serde_json::Value = load_frequency();
}

/// The `Guesser` struct provides an interface to the searching algorithm
pub struct Guesser {
    searches: i32,
    index: InfoIndex,
    difficulty: usize,
    attempt: usize,
}


impl Guesser {
    /// Returns a new Guesser given a difficulty and current info
    pub fn new(difficulty: usize, all_info: &AllInfo) -> Guesser {
        let index = InfoIndex::build(difficulty, all_info);
        Guesser {
            searches: 0,
            index,
            difficulty,
            attempt: all_info.len(),
        }
    }

    /// Returns a sorted character array given a mutable choice array
    /// The first entry is the highest rated character
    fn choice_sort(
        &self,
        choices: &mut Vec<char>,
        position: usize,
        frequency: &HashMap<char, usize>,
    ) {
        let mut rng = rand::thread_rng();

        let lookup = &FREQUENCY[&self.difficulty.to_string()][&position.to_string()];
        choices.sort_by_cached_key(|c| {
            let value = if frequency.is_empty() {
                0.1 * rng.gen::<f64>() - lookup[&c.to_string()].as_f64().unwrap()
            } else {
                0.1 * rng.gen::<f64>() - 5.0 * lookup[&c.to_string()].as_f64().unwrap() + frequency[&c] as f64
            };


            return (value * 1000000.0) as i32;
        });
    }

    /// Returns the initial guess from the guesser
    fn initial_guess(&self) -> String {
        let mut nums: Vec<char> = NUMS.to_vec();

        // reasons
        if self.difficulty >= 8 {
            nums.push('%');
        }
        if self.difficulty >= 13 {
            nums.extend(OPERATORS.iter());
        }
        if self.difficulty >= 14 {
            nums.push('9');
        }

        let mut template: Vec<char> = OPTIMAL_GUESSES[self.difficulty - 7].chars().collect();

        let mut frequency = HashMap::new();
        for key in CHARS.iter() {
            frequency.insert(key.clone(), 0);
        }

        // replace stuff with random
        for (i, target) in template.iter_mut().enumerate() {
            if *target == '=' {
                continue;
            }

            self.choice_sort(
                &mut nums,
                i,
                &frequency,
            );
            *frequency.get_mut(&nums[0]).unwrap() += 1;
            *target = nums[0];
        }

        template.iter().collect()
    }

    /// Return the length of the result by deducing the equality operator position
    fn deduce_equality(&mut self) -> usize {
        let result_length: usize;

        let equality = self.index.lookup.get_mut(&'=').unwrap();
        if equality.correct.len() != 1 {
            let position = min(self.difficulty - 2, 8);

            equality.correct.clear();
            equality.correct.insert(position);

            result_length = self.difficulty - position - 1;
        } else {
            result_length = self.difficulty - equality.correct.iter().next().unwrap() - 1;
        }

        result_length
    }

    /// Returns if the guesser should attempt to fall at this stage
    fn should_fail(&self) -> bool {
        // count greens
        let mut correct = 0;
        let mut correct_operators = vec![];
        for (key, value) in self.index.lookup.iter() {
            correct += value.correct.len();
            if OPERATORS.contains(key) && value.min > 0 {
                correct_operators.push(key.clone());
            }
        }

        let diff = self.difficulty;
        match self.difficulty {
            7 => {
                return false;
            }
            8 => {
                return self.attempt == 1;
            }
            9 => {
                if self.attempt == 1 {
                    return true;
                }

                if self.index.lookup[&'%'].min > 0 && (diff - correct <= 4) {
                    if self.attempt == 2 {
                        return true;
                    }
                }
            }
            10 => {
                if self.attempt == 1 {
                    return true;
                }

                if self.attempt == 3 && correct_operators.contains(&'%')
                    && correct_operators.len() == 1
                    && 1 < (diff - correct)
                    && (diff - correct) <= 3 {
                    return true;
                }
            }
            _ => {
                if self.attempt == 1 {
                    return true;
                }

                if self.attempt == 2 && (diff - correct) <= 2 {
                    return true;
                }


                if self.attempt == 3 && correct_operators.contains(&'%')
                    && correct_operators.len() == 1
                    && 1 < (diff - correct)
                    && (diff - correct) <= 3 {
                    return true;
                }
            }
        }

        false
    }

    /// Returns some helpful lookup caching data structures
    fn create_guess_variables(&self)
                              -> (Vec<(char, &InfoLookup)>, HashMap<usize, char>, usize) {
        let mut valid: Vec<(char, &InfoLookup)> = Vec::new();
        let mut valid_positions: HashMap<usize, char> = HashMap::new();
        let mut valid_operators = 0;

        for (key, lookup) in self.index.lookup.iter() {
            if *key == '=' {
                continue;
            }

            if lookup.max == 0 {
                continue;
            }

            valid.push((*key, lookup));

            for i in lookup.correct.iter() {
                valid_positions.insert(*i, *key);
            }

            if OPERATORS.contains(key) {
                valid_operators += lookup.correct.len();
            }
        }

        (valid, valid_positions, valid_operators)
    }

    fn fail_filters(
        &self,
        position: usize, output: &Vec<char>,
        ops_left: i32, ops_valid: usize,
    ) -> HashSet<char> {
        let mut filter = HashSet::new();


        if position == 0 {
            filter.extend(&[
                '+', '-', '*', '%', '0'
            ]);
        }

        if position + 1 == self.difficulty {
            filter.extend(&OPERATORS);
        }

        if position >= 1 && OPERATORS.contains(output.last().unwrap()) {
            filter.extend(&[
                '+', '-', '*', '%', '0'
            ]);
        }

        if ops_left <= -2 {
            filter.extend(&OPERATORS);
        }

        if ops_valid >= 2 {
            filter.extend(&OPERATORS);
        }

        filter
    }

    fn normal_filters(
        &self,
        position: usize, output: &Vec<char>,
        ops_left: i32, ops_valid: usize,
        chars_remain: usize,
    ) -> HashSet<char> {
        let mut filter = HashSet::new();

        if position == 0 {
            filter.extend(&[
                '+', '-', '*', '%', '0'
            ]);
        }

        if position + 1 == self.difficulty {
            filter.extend(&OPERATORS);
        }

        if position >= 1 && OPERATORS.contains(output.last().unwrap()) {
            filter.extend(&[
                '+', '-', '*', '%', '0'
            ]);
        }

        if position >= 2
            && NUMS.contains(output.last().unwrap())
            && NUMS.contains(&output[output.len() - 2]) {
            filter.extend(&NUMS);
        }

        if chars_remain == 2
            && NUMS.contains(output.last().unwrap()) {
            filter.extend(&NUMS);
        }


        if (chars_remain == 2 && ops_left == 1)
            || (chars_remain == 3 && ops_left == 2) {
            filter.extend(&NUMS);
        }

        if (ops_left == 0) || ops_valid >= 3 {
            filter.extend(&OPERATORS);
        }

        filter
    }


    /// Returns the best guess the guesser could possibly make.
    pub fn create_guess(&mut self) -> String {
        // initial guess
        if self.attempt == 0 {
            return self.initial_guess();
        }

        let result_length = self.deduce_equality();
        let expression_length = self.difficulty - result_length - 1;

        let fail = self.should_fail();
        let (valid, valid_positions, valid_operators) = self.create_guess_variables();

        return if fail {
            self.backtrack_fail(
                valid,
                valid_positions,
                valid_operators,
                expression_length,
                result_length,
            )
        } else {
            self.backtrack(
                valid,
                valid_positions,
                valid_operators,
                expression_length,
                result_length,
            )
        };
    }

    /// Returns the best guess by failing all the valid positions
    fn backtrack_fail(
        &self,
        valid: Vec<(char, &InfoLookup)>,
        valid_positions: HashMap<usize, char>,
        valid_operators: usize,
        expression_length: usize,
        result_length: usize,
    ) -> String {

        // create output array
        // this is to avoid recursion
        // the last char in each entry is the "best" guess
        let mut stack: Vec<Vec<char>> = vec![];

        // pre populate output
        for _ in 0..self.difficulty {
            stack.push(vec![]);
        }


        let mut output_frequency: HashMap<char, usize> = HashMap::new();

        loop {
            // state variable: current guess ops left
            let mut ops_left: i32 = 2;
            // state variable
            for key in CHARS.iter() {
                output_frequency.insert(key.clone(), 0);
            }

            // add top entries --- the current partial guess
            let mut output = vec![];
            for choices in stack.iter() {
                if choices.len() > 0 {
                    let item = choices.last().unwrap().clone();

                    if OPERATORS.contains(&item) {
                        ops_left -= 1;
                    }

                    output.push(item);
                    *output_frequency.get_mut(&item).unwrap() += 1;
                } else {
                    break;
                }
            }

            // cursor position
            let position = output.len();

            // return if output is full
            if position == self.difficulty {
                return output.iter().collect();
            }

            // create filter
            let mut filter = self.fail_filters(
                position,
                &output,
                ops_left,
                valid_operators,
            );

            // avoid using correct positions
            if valid_positions.contains_key(&position) {
                filter.insert(
                    valid_positions.get(&position).unwrap().clone()
                );
            }

            // ignore operators when % is present
            if self.index.lookup.get(&'%').unwrap().min >= 1 {
                filter.extend(&OPERATORS);
            }

            // compute next position choices
            // the target empty vec
            let mut choices = &mut stack[position];
            for (key, lookup) in valid.iter() {
                // enact filter
                if filter.contains(key) {
                    continue;
                }

                // don't place incorrect values
                if lookup.incorrect.contains(&position) {
                    continue;
                }

                // don't add if we exceeded character maximum
                let char_freq = output_frequency.get(&key).unwrap().clone();
                let too_much = (lookup.max == lookup.min) && (lookup.min <= char_freq);
                if too_much {
                    continue;
                }

                choices.push(key.clone());
            }


            // ignore when no choices are found
            if choices.len() == 0 {
                if valid_positions.contains_key(&position) {
                    // try the valid position
                    choices.push(valid_positions[&position]);
                } else {
                    // or fallback to all characters
                    choices.extend(NUMS.iter());
                }
            }


            let mut subtracted_freq: HashMap<char, usize> = HashMap::new();
            for (key, value) in output_frequency.iter() {
                let correct = self.index.lookup[key].correct.len();
                subtracted_freq.insert(
                    key.clone(),
                    value + self.difficulty - correct,
                );
            }

            // sort choices
            self.choice_sort(
                &mut choices,
                position,
                &subtracted_freq,
            );

            // flip because we want the best character to be on the top of
            // the stack, ie, the last element
            choices.reverse();

            // we already appended it, so just move on
        }

        panic!("failed to find a fail solution");

        // backup
        "0".repeat(self.difficulty)
    }

    // backtrack function
    // takes the current position, and the stack
    // returns whether we have ran out of places
    fn revert(&self, position: usize, stack: &mut Vec<Vec<char>>) -> bool {
        let mut position = position as i32 - 1;

        while position >= 0 {
            stack[position as usize].pop();

            if stack[position as usize].len() == 0 {
                position -= 1;
                continue;
            }

            break;
        }

        // valid iff position is not negative
        position >= 0
    }

    fn backtrack(
        &self,
        valid: Vec<(char, &InfoLookup)>,
        valid_positions: HashMap<usize, char>,
        valid_operators: usize,
        expression_length: usize,
        result_length: usize,
    ) -> String {

        // create output array
        // this is to avoid recursion
        // the last char in each entry is the "best" guess
        let mut stack: Vec<Vec<char>> = vec![];
        for _ in 0..expression_length {
            stack.push(vec![]);
        }

        let mut output_frequency: HashMap<char, usize> = HashMap::new();

        'outer:
        loop {
            // state variable: current guess ops left
            let mut ops_left: i32 = 2;
            // state variable: output characters freq
            for key in CHARS.iter() {
                output_frequency.insert(key.clone(), 0);
            }

            // add top entries --- the current partial guess
            let mut output = vec![];
            for choices in stack.iter() {
                if choices.len() > 0 {
                    let item = choices.last().unwrap().clone();

                    if OPERATORS.contains(&item) {
                        ops_left -= 1;
                    }

                    output.push(item);
                    *output_frequency.get_mut(&item).unwrap() += 1;
                } else {
                    break;
                }
            }

            // cursor position
            let position = output.len();

            // check if output is full
            if position == expression_length {
                let expression: String = output.iter().collect();

                let answer = fast_eval(&expression);
                if answer.is_none()
                    || answer.unwrap() <= 0 {
                    // keep searching
                    if !self.revert(position, &mut stack) {
                        break 'outer;
                    }
                    continue 'outer;
                }

                let answer = answer.unwrap().to_string();
                if answer.len() != result_length {
                    // keep searching
                    if !self.revert(position, &mut stack) {
                        break 'outer;
                    }
                    continue 'outer;
                }

                let guess = format!("{}={}", expression, answer);
                let passed = passes_restrictions(
                    &guess,
                    &self.index,
                );

                if !passed {
                    // keep searching
                    if !self.revert(position, &mut stack) {
                        break 'outer;
                    }
                    continue 'outer;
                }

                return guess;
            }

            // create filter
            let filter = self.normal_filters(
                position,
                &output,
                ops_left,
                valid_operators,
                expression_length - (position),
            );


            let mut choices = &mut stack[position];

            // use correct positions
            if valid_positions.contains_key(&position) {
                let key = valid_positions[&position];

                let lookup = &self.index.lookup[&key];
                let char_freq = output_frequency.get(&key).unwrap().clone();
                let too_much = (lookup.max == lookup.min) && (lookup.min <= char_freq);
                if filter.contains(&key) || too_much {
                    // backtrack
                    if !self.revert(position, &mut stack) {
                        break 'outer;
                    }
                    continue 'outer;
                }

                choices.push(key);
                continue 'outer;
            }


            // compute next position choices
            // the target empty vec
            for (key, lookup) in valid.iter() {
                // enact filter
                if filter.contains(key) {
                    continue;
                }

                // don't place incorrect values
                if lookup.incorrect.contains(&position) {
                    continue;
                }

                // don't add if we exceeded character maximum
                let char_freq = output_frequency.get(&key).unwrap().clone();
                let too_much = (lookup.max == lookup.min) && (lookup.min <= char_freq);
                if too_much {
                    continue;
                }

                choices.push(key.clone());
            }


            // ignore when no choices are found
            if choices.len() == 0 {
                if !self.revert(position, &mut stack) {
                    break 'outer;
                }
                continue 'outer;
            }

            // sort choices
            self.choice_sort(
                &mut choices,
                position,
                &HashMap::new(),
            );

            // flip because we want the best character to be on the top of
            // the stack, ie, the last element
            choices.reverse();

            // we already appended it, so just move on
        }

        panic!("failed to find a solution");

        // backup
        "0".repeat(self.difficulty)
    }
}