/*
Wordle Bot - a solver for the online game "Wordle"
Copyright (C) 2022 Joshua Williams

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::env;
use std::fs::File;
use std::io::{self, stdin, BufRead, Write};

// The word length is hardcoded so that we can store buffers of the right length on the stack
const WORD_LENGTH: usize = 5;
const POSSIBLE_FEEDBACKS: usize = 3_usize.pow(WORD_LENGTH as u32);

// Define these types for easy modification
type Feedback = u32;
type Score = f32;

struct WordleDictionary {
    n_words: usize,
    n_solutions: usize,
    solutions: Vec<String>,
    all_words: Vec<String>,
    feedbacks: Vec<Vec<Feedback>>,
    solutions_to_words: Vec<usize>,
}

impl WordleDictionary {
    fn new(all_words: Vec<String>, solutions: Vec<String>) -> Self {
        let n_words = all_words.len();
        let n_solutions = solutions.len();
        let mut dictionary = WordleDictionary {
            n_words,
            n_solutions,
            all_words,
            solutions,
            feedbacks: Vec::new(),
            solutions_to_words: Vec::new(),
        };
        dictionary.calculate_solutions_to_words();
        dictionary.calculate_feedbacks();
        dictionary
    }

    fn calculate_solutions_to_words(&mut self) {
        self.solutions_to_words = self
            .solutions
            .iter()
            .map(|solution_string| {
                self.all_words
                    .iter()
                    .enumerate()
                    .find(|(_, word_string)| *word_string == solution_string)
                    .unwrap()
                    .0
            })
            .collect();
    }

    fn solution_to_word(&self, solution: usize) -> usize {
        self.solutions_to_words[solution]
    }

    fn calculate_feedbacks(&mut self) {
        self.feedbacks = (0..self.n_words)
            .into_iter()
            .map(|guess| {
                (0..self.n_solutions)
                    .map(|solution| self.calculate_feedback(solution, guess))
                    .collect()
            })
            .collect();
    }

    fn calculate_feedback(&self, word: usize, guess: usize) -> Feedback {
        let word = self.solutions[word].as_bytes();
        let guess = self.all_words[guess].as_bytes();
        let mut used = [false; WORD_LENGTH];
        let mut correct = [false; WORD_LENGTH];
        let mut contained = [false; WORD_LENGTH];
        // Correctly placed check
        for i in 0..WORD_LENGTH {
            if word[i] == guess[i] {
                used[i] = true;
                correct[i] = true;
            }
        }
        // Incorrectly placed check
        for i in 0..WORD_LENGTH {
            if correct[i] {
                continue;
            }
            for j in 0..WORD_LENGTH {
                if used[j] {
                    continue;
                }
                if guess[i] == word[j] {
                    contained[i] = true;
                    used[j] = true;
                    break;
                }
            }
        }
        // Convert these arrays to a numerical feedback
        let mut feedback = 0;
        for i in 0..WORD_LENGTH {
            feedback *= 3;
            if correct[i] {
                feedback += 2;
            } else if contained[i] {
                feedback += 1;
            }
        }
        feedback
    }

    fn get_feedback(&self, solution: usize, guess: usize) -> Feedback {
        self.feedbacks[guess][solution]
    }

    fn solution_string(&self, word: usize) -> &String {
        &self.solutions[word]
    }

    fn word_string(&self, word: usize) -> &String {
        &self.all_words[word]
    }
}

struct WordleBot {
    dictionary: WordleDictionary,
    remaining_set: Vec<usize>,
    first_guess: usize,
    is_first_guess: bool,
}

impl WordleBot {
    fn new(dictionary: WordleDictionary) -> Self {
        let n_solutions = dictionary.n_solutions;
        let mut bot = WordleBot {
            dictionary,
            remaining_set: (0..n_solutions).collect(),
            first_guess: 0,
            is_first_guess: false,
        };
        bot.calculate_first_guess();
        bot
    }

    fn calculate_first_guess(&mut self) {
        self.first_guess = self.get_guess();
        self.is_first_guess = true;
    }

    fn give_feedback(&mut self, word: usize, feedback: Feedback) {
        self.is_first_guess = false;
        let feedbacks = &self.dictionary.feedbacks[word];
        self.remaining_set.retain(|&x| feedbacks[x] == feedback);
    }

    fn get_guess(&self) -> usize {
        if let Option::Some(solution) = self.get_solution() {
            self.dictionary.solution_to_word(solution)
        } else if self.is_first_guess {
            self.first_guess
        } else {
            (0..self.dictionary.n_words)
                .into_iter()
                .map(|word| (word, self.score(word)))
                .reduce(|x, y| if x.1 < y.1 {x} else {y})
                .expect("Error: No guessable words!")
                .0
        }
    }

    fn score(&self, guess: usize) -> Score {
        let feedbacks = &self.dictionary.feedbacks[guess];
        let mut class_sizes: [u32; POSSIBLE_FEEDBACKS] = [0; POSSIBLE_FEEDBACKS];
        for &possible_solution in self.remaining_set.iter() {
            class_sizes[feedbacks[possible_solution] as usize] += 1;
        }
        class_sizes
            .into_iter()
            .take(POSSIBLE_FEEDBACKS - 1)
            .map(|class_size| class_size as f32)
            .map(|class_size| (class_size * (class_size + 1.0).log2()) as Score)
            .sum()
    }

    fn get_solution(&self) -> Option<usize> {
        if self.remaining_set.len() == 1 {
            Option::Some(self.remaining_set[0])
        } else {
            Option::None
        }
    }

    fn reset(&mut self) {
        self.is_first_guess = true;
        self.remaining_set = (0..self.dictionary.n_solutions).collect();
    }
}

fn feedback_from_string(input: &str) -> Feedback {
    let input = input.as_bytes();
    let mut feedback = 0;
    for &c in input.iter() {
        feedback *= 3;
        if c == b'g' {
            feedback += 2;
        } else if c == b'y' {
            feedback += 1;
        }
    }
    feedback
}

fn display_average_guesses(mut bot: WordleBot) {
    let mut total_guesses = 0;
    for solution in 0..bot.dictionary.n_solutions {
        bot.reset();
        let solution_as_word = bot.dictionary.solution_to_word(solution);
        loop {
            let guess: usize = bot.get_guess();
            total_guesses += 1;
            if guess == solution_as_word {
                break;
            }
            bot.give_feedback(guess, bot.dictionary.get_feedback(solution, guess));
        }
    }
    let average_guesses = (total_guesses as f64) / (bot.dictionary.n_solutions as f64);
    println!("Average guesses used: {average_guesses}");
}

fn play_game(mut bot: WordleBot) {
    loop {
        match bot.get_solution() {
            Option::Some(solution) => {
                let solution_string = bot.dictionary.solution_string(solution);
                println!("Solution found: {solution_string}");
                break;
            }
            Option::None => (),
        }
        let guess = bot.get_guess();
        let guess_string = bot.dictionary.word_string(guess);

        println!("My guess is {guess_string}",);
        print!("Please enter feedback using 'g', 'y', and 'x': ");
        io::stdout().flush().unwrap();
        let mut feedback = String::new();
        stdin().read_line(&mut feedback).unwrap();
        println!();
        let feedback = feedback.trim();
        let feedback = feedback_from_string(feedback);

        bot.give_feedback(guess, feedback);
    }
}

fn main() {
    let mut args = env::args();
    let _path_to_executable = args.next();
    let mode = args.next().unwrap_or_else(|| "game".to_string());
    let solutions_filename = args
        .next()
        .unwrap_or_else(|| "dictionary_solutions.txt".to_string());
    let dictionary_filename = args
        .next()
        .unwrap_or_else(|| "dictionary_full.txt".to_string());

    let solutions: Vec<String> = io::BufReader::new(File::open(solutions_filename).unwrap())
        .lines()
        .map(|x| x.unwrap())
        .collect();
    let all_words: Vec<String> = io::BufReader::new(File::open(dictionary_filename).unwrap())
        .lines()
        .map(|x| x.unwrap())
        .collect();
    let bot = WordleBot::new(WordleDictionary::new(all_words, solutions));

    match mode.as_str() {
        "average" => display_average_guesses(bot),
        _ => play_game(bot),
    }
}
