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

pub type Feedback = u32;

// Some useful constants
const NUM_LETTERS: usize = 26;
const FIRST_LETTER: u8 = b'a';

pub fn calculate_feedback(target: &[u8], guess: &[u8]) -> Feedback {
    let word_length = target.len();

    // Check for error conditions
    if 3_u32.pow(word_length as u32) > Feedback::max_value() {
        panic!("Target string is too long for current feedback type!");
    }
    if guess.len() != word_length {
        panic!("Target and guess must be the same length!");
    }

    // Build a histogram of letters in the target and find correctly placed letters in one pass
    let mut unused_letters = [0; NUM_LETTERS];
    let mut correct_feedback = 0;
    target.iter().zip(guess.iter()).for_each(|characters| {
        correct_feedback *= 3;
        if characters.0 == characters.1 {
            correct_feedback += 2;
        } else {
            unused_letters[(characters.0 - FIRST_LETTER) as usize] += 1;
        }
    });

    // Use the histogram to find correct letters that are incorrectly placed
    let mut incorrect_feedback = 0;
    target.iter().zip(guess.iter()).for_each(|characters| {
        incorrect_feedback *= 3;
        if characters.0 != characters.1 {
            let letter = (characters.1 - FIRST_LETTER) as usize;
            if unused_letters[letter] > 0 {
                unused_letters[letter] -= 1;
                incorrect_feedback += 1;
            }
        }
    });
    correct_feedback + incorrect_feedback
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_wrong() {
        assert_eq!(calculate_feedback(b"soare", b"clint"), 0);
        assert_eq!(calculate_feedback(b"alpha", b"every"), 0);
        assert_eq!(calculate_feedback(b"aaaaa", b"bbbbb"), 0);
    }

    // Test cases where some characters are in the correct position
    #[test]
    fn test_some_correct() {
        assert_eq!(calculate_feedback(b"hello", b"zebra"), 54);
        assert_eq!(calculate_feedback(b"abxyz", b"abcde"), 216);
    }

    // Test cases where all characters are correct
    #[test]
    fn test_all_correct() {
        assert_eq!(calculate_feedback(b"abcde", b"abcde"), 242);
        assert_eq!(calculate_feedback(b"aaaaa", b"aaaaa"), 242);
    }

    // Test cases where some characters are in incorrect positions
    #[test]
    fn test_some_incorrect() {
        assert_eq!(calculate_feedback(b"abcde", b"bcdea"), 121);
        assert_eq!(calculate_feedback(b"hello", b"soare"), 28);
    }

    // Test cases where there are a mix of letters correctly and incorrectly placed
    #[test]
    fn test_complex() {
        assert_eq!(calculate_feedback(b"abcde", b"cbayz"), 144);
        assert_eq!(calculate_feedback(b"abcde", b"xyzdd"), 6);
        assert_eq!(calculate_feedback(b"yyddd", b"dddxx"), 126);
    }
}
