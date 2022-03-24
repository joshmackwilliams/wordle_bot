# wordle_bot

A bot for the online game "Wordle"

### Usage

You will need the latest (stable) version of Cargo to compile and run this
program. Download the code and run it using `cargo run --release`. It's
important for performance to use the release profile - this project benefits
significantly from compiler optimizations.

The program will provide you with guesses, and you must provide feedback. To do
this, enter a string consisting of the characters 'x', 'y', and 'g', which
correspond to grey, yellow and green. The bot will process your input and
provide another guess until only one possible word remains.

### How it works

The bot chooses the optimal guess to minimize the expected value of the
logarithm of remaining possibilities after making the guess. This is because the
number of guesses required to finish a game will grow roughly with the logarithm
of remaining possible solutions, so by minimizing this value, we roughly
minimize the number of guesses we still need in order to finish the game.
