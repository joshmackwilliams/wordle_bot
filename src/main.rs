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

use std::fs::File;
use std::io::{self, BufRead, stdin, Write};
use rayon::iter::{ IntoParallelIterator, IntoParallelRefIterator, ParallelIterator };

// The word length is hardcoded so that we can store buffers of the right length on the stack
const WORD_LENGTH: usize = 5;
const POSSIBLE_SIGNATURES: usize = 3_usize.pow(WORD_LENGTH as u32);

const CHARACTERS_PER_LINE: usize = 80;

// Define these types for easy modification
type Signature = u32;
type Score = u32;

fn get_signature(word: &str, guess: &str) -> Signature {
    let word = word.as_bytes();
    let guess = guess.as_bytes();
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
    // Convert these arrays to a numerical signature
    let mut signature = 0;
    for i in 0..WORD_LENGTH {
        signature *= 3;
        if correct[i] {
            signature += 2;
        } else if contained[i] {
            signature += 1;
        }
    }
    signature
}

fn to_signature(input: &str) -> Signature {
    let input = input.as_bytes();
    let mut signature = 0;
    for i in 0..WORD_LENGTH {
        signature *= 3;
        if input[i] == b'g' {
            signature += 2;
        } else if input[i] == b'y' {
            signature += 1;
        }
    }
    signature
}

fn score(guess: &str, possible_solutions: &[String]) -> Score {
    let mut group_sizes: [Score; POSSIBLE_SIGNATURES] = [0; POSSIBLE_SIGNATURES];
    for word in possible_solutions {
        let signature = get_signature(word, guess);
        group_sizes[signature as usize] += 1;
    }
    let mut score = 0;
    // We don't count the last signature, which is "ggggg"
    for i in 0..group_sizes.len() - 1 {
        let group_size = group_sizes[i];
        score += group_size * group_size;
    }
    score
}

// Prints a vector of words in columns
fn print_words(words: &[String]) {
    for i in 0..words.len() {
        print!("{} ", words[i]);
        if (i + 1) % (CHARACTERS_PER_LINE / 6) == 0 {
            println!("");
        }
    }
    println!("");
}

fn main() {
    let mut possible_solutions: Vec<String> =
        io::BufReader::new(File::open("dictionary_solutions.txt").unwrap())
            .lines()
            .map(|x| x.unwrap())
            .collect();
    let guessable_words: Vec<String> =
        io::BufReader::new(File::open("dictionary_full.txt").unwrap())
            .lines()
            .map(|x| x.unwrap())
            .collect();
    println!(
        "{} guessable words, {} possible solutions",
        guessable_words.len(),
        possible_solutions.len()
    );
    loop {
        // Guess
        let guess = guessable_words
            .par_iter()
            .map(|word| (word, score(word, &possible_solutions)))
            .min_by(|x, y| x.1.cmp(&y.1))
            .expect("Error: No guessable words!");
        println!("My guess is {}, with a score of {}", guess.0, guess.1);

        // Get feedback from user
        print!("Please enter feedback using 'g', 'y', and 'x': ");
        io::stdout().flush().unwrap();
        let mut signature = String::new();
        stdin().read_line(&mut signature).unwrap();
        println!("");
        let signature = signature.trim();

        // Trim down solutions based on feedback
        let signature = to_signature(signature);
        possible_solutions = possible_solutions
            .into_par_iter()
            .filter(|word| -> bool { signature == get_signature(word, guess.0) })
            .collect();

        // Check for end conditions and print information
        if possible_solutions.len() == 1 {
            println!("Word found: {}", possible_solutions[0]);
            break;
        } else if possible_solutions.is_empty() {
            println!("Error: No remaining possible words!");
            break;
        } else {
            println!("There are {} remaining possible solutions: ", possible_solutions.len());
            print_words(&possible_solutions);
            println!("");
        }
    }
}
