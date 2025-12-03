# Advent of Code 2025
My Rust implementations of the 2025 [Advent of Code](https://adventofcode.com/) challenges.

## Usage
```bash
aoc25 <DAY> [-d --disable <OPTIONS>...]
```
The tool will run four tests for the day specified, aligning with the Advent of Code tests: "Example 1," "Puzzle 1," "Example 2," and "Puzzle 2."
Any one of these tests may be skipped either through the `--disable` option or by not providing the test file. Test files are located in "challenge_files";
the examples will read from "challenge_files/examples/day{}.txt", and puzzles from "challenge_files/puzzles/day{}.txt", where "{}" will be replaced with
the day being tested.
