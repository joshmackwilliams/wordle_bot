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

use wordle_bot::feedback_calculator::Feedback;
use wordle_bot::wordle_bot::WordleBot;
use wordle_bot::wordle_dictionary::WordleDictionary;

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
    for solution in 0..bot.get_dictionary().get_n_solutions() {
        bot.reset();
        loop {
            let guess: usize = bot.get_guess();
            total_guesses += 1;
            if guess == solution {
                break;
            }
            bot.give_feedback(guess, bot.get_dictionary().get_feedback(solution, guess));
        }
    }
    let average_guesses = (total_guesses as f64) / (bot.get_dictionary().get_n_solutions() as f64);
    println!("Average guesses used: {average_guesses}");
}

fn play_game(mut bot: WordleBot) {
    loop {
        match bot.get_solution() {
            Option::Some(solution) => {
                let solution_string = bot.get_dictionary().word_string(solution);
                println!("Solution found: {solution_string}");
                break;
            }
            Option::None => (),
        }
        let guess = bot.get_guess();
        let guess_string = bot.get_dictionary().word_string(guess);

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
