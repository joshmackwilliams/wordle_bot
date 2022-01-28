# wordle_bot

A bot for the online game "Wordle"

### Usage

You will need the latest (stable) version of Cargo to compile and run
this program. Download the code and run it using `cargo run
--release`. It's important for performance to use the release profile
- this project benefits significantly from compiler optimizations.

The program will provide you with guesses, and you must provide
feedback. To do this, enter a string consisting of the characters 'x',
'y', and 'g', which correspond to grey, yellow and green. The bot will
process your input and provide another guess until only one possible
word remains.

### How it works

The bot simply chooses the optimal guess to minimize the expected
number of remaining possibilities. For now, it's no more complicated
than that.

### Upcoming Features

- Hard Mode: The existing algorithm will perform very poorly on hard
  mode. So, if the bot is expanded to work in hard mode, it will
  likely do an exhaustive search to find the actual optimal solution
  rather than using heuristics as described above.

- Performance Improvements: The bot is already quite fast, but could
  benefit from some caching-related features.

- More Advanced Search: We still have computing power to spare, and
  hopefully we'll have even more after some performance
  improvements. So, that leaves the option of potentially using a more
  advanced search algorithm. This could possibly mean looking ahead
  one or more guesses. Depending on how much compute power all of this
  takes, finding the optimal guess pattern is a possibility.
