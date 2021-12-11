# AdventOfCode

Repo for Advent of Code challenges.

I'm primarily doing these for fun and general experience with Rust, with a smattering of learning new stuff on the side.

Testing a commit

## Contents:
- 2019
  - Most solutions for 2019, in a fairly scattershot arrangement (most of the puzzles involving the computer are in all all_days, other puzzles are their own cargo crates)
  - Would like, at some point, to go back and (a) clean this up and (b) complete the missing puzzles!
- 2020
  - Almost all solutions for 2020, in a more organised format.
  - ~6 of the 49 puzzles do not directly output a solution, at least based on rerunning the code on a new account's input and finding ones which returned 0 or gave the wrong answer.
  - Additionally, Day 8 requires hard-coding of the number of lines in the input file, which is inelegant.
- template
  - Framework files for a new year.  Feel free to clone!
  - Thoughts for improvement:
    - Add test framework for the template
    - Add bit to script to get the inputs automatically each day/when first run after the given day
    - Add some particularly common utilities and structs e.g. Coords, various initial parsings of input_lines.
    - Change default variable type for output of functions to i32.  I'm not sure I've ever seen a negative answer, but generally handling i32s is better (they allow subtraction!) through the code.  Or maybe to a string, so we can handle the odd occasion where a string is needed!
- year21
  - Complete without caveats at point of writing (11th December 2021). Should correctly answer for any given input!
  - All the template thoughts for improvement apply to this codebase (apart from adding top level tests, which I've done on a day-by-day basis).
