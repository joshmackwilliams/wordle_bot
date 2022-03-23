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

use crate::feedback_calculator::Feedback;
use crate::wordle_dictionary::WordleDictionary;

pub type Score = f32;

const WORD_LENGTH: usize = 5;
const POSSIBLE_FEEDBACKS: usize = 3_usize.pow(WORD_LENGTH as u32);

pub struct WordleBot {
    dictionary: WordleDictionary,
    remaining_set: Vec<usize>,
    first_guess: usize,
    is_first_guess: bool,
}

impl WordleBot {
    pub fn new(dictionary: WordleDictionary) -> Self {
        let n_solutions = dictionary.get_n_solutions();
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

    pub fn give_feedback(&mut self, word: usize, feedback: Feedback) {
        self.is_first_guess = false;
        self.remaining_set
            .retain(|&x| self.dictionary.get_feedback(x, word) == feedback);
    }

    pub fn get_guess(&self) -> usize {
        if let Option::Some(solution) = self.get_solution() {
            solution
        } else if self.is_first_guess {
            self.first_guess
        } else {
            (0..self.dictionary.get_n_words())
                .into_iter()
                .map(|word| (word, self.score(word)))
                .reduce(|x, y| if x.1 < y.1 { x } else { y })
                .expect("Error: No guessable words!")
                .0
        }
    }

    fn score(&self, guess: usize) -> Score {
        let mut class_sizes: [u32; POSSIBLE_FEEDBACKS] = [0; POSSIBLE_FEEDBACKS];
        for &possible_solution in self.remaining_set.iter() {
            class_sizes[self.dictionary.get_feedback(possible_solution, guess) as usize] += 1;
        }
        class_sizes
            .into_iter()
            .take(POSSIBLE_FEEDBACKS - 1)
            .map(|class_size| class_size as f32)
            .map(|class_size| (class_size * (class_size + 1.0).log2()) as Score)
            .sum()
    }

    pub fn get_solution(&self) -> Option<usize> {
        if self.remaining_set.len() == 1 {
            Option::Some(self.remaining_set[0])
        } else {
            Option::None
        }
    }

    pub fn reset(&mut self) {
        self.is_first_guess = true;
        self.remaining_set = (0..self.dictionary.get_n_solutions()).collect();
    }

    pub fn get_dictionary(&self) -> &WordleDictionary {
        &self.dictionary
    }
}
