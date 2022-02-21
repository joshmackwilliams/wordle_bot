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

use crate::types::Feedback;

pub struct WordleDictionary<const WORD_LENGTH: usize> {
    n_words: usize,
    n_solutions: usize,
    solutions: Vec<String>,
    all_words: Vec<String>,
    feedbacks: Vec<Vec<Feedback>>,
    solutions_to_words: Vec<usize>,
}

impl<const WORD_LENGTH: usize> WordleDictionary<WORD_LENGTH> {
    pub fn new(all_words: Vec<String>, solutions: Vec<String>) -> Self {
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

    pub fn solution_to_word(&self, solution: usize) -> usize {
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

    pub fn get_feedback(&self, solution: usize, guess: usize) -> Feedback {
        self.feedbacks[guess][solution]
    }

    pub fn solution_string(&self, word: usize) -> &String {
        &self.solutions[word]
    }

    pub fn word_string(&self, word: usize) -> &String {
        &self.all_words[word]
    }

    pub fn get_n_words(&self) -> usize {
        self.n_words
    }

    pub fn get_n_solutions(&self) -> usize {
        self.n_solutions
    }
}
