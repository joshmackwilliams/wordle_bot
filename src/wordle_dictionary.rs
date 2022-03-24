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

use crate::feedback_calculator::calculate_feedback;
use crate::feedback_calculator::Feedback;
use std::collections::HashSet;

pub struct WordleDictionary {
    n_solutions: usize,
    words: Vec<String>,
    feedbacks: Vec<Feedback>,
}

impl WordleDictionary {
    pub fn new(guessable_words: Vec<String>, solutions: Vec<String>) -> Self {
        let n_solutions = solutions.len();
        let solutions_set: HashSet<String> = HashSet::from_iter(solutions.iter().cloned());
        let words: Vec<String> = solutions
            .into_iter()
            .chain(
                guessable_words
                    .into_iter()
                    .filter(|x| !solutions_set.contains(x)),
            )
            .collect();
        let feedbacks: Vec<Feedback> = words
            .iter()
            .flat_map(|guess| {
                words
                    .iter()
                    .take(n_solutions)
                    .map(|solution| calculate_feedback(solution.as_bytes(), guess.as_bytes()))
            })
            .collect();
        WordleDictionary {
            n_solutions,
            words,
            feedbacks,
        }
    }

    pub fn get_feedback(&self, solution: usize, guess: usize) -> Feedback {
        self.feedbacks[(guess * self.n_solutions) + solution]
    }

    pub fn word_string(&self, word: usize) -> &String {
        &self.words[word]
    }

    pub fn get_n_words(&self) -> usize {
        self.words.len()
    }

    pub fn get_n_solutions(&self) -> usize {
        self.n_solutions
    }
}
