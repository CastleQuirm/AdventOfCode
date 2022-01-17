# AdventOfCode

Repo for Advent of Code challenges.

I'm primarily doing these for fun and general experience with Rust, with a smattering of learning new stuff on the side.

## Contents:
- 2019
  - Most solutions for 2019, in a fairly scattershot arrangement (most of the puzzles involving the computer are in all all_days, other puzzles are their own cargo crates)
  - Would like, at some point, to go back and (a) clean this up and (b) complete the missing puzzles!
- template
  - Framework files for a new year.  Feel free to clone!
  - Thoughts for improvement:
    - Add test framework for the template
    - Add bit to script to get the inputs automatically each day/when first run after the given day
    - Add some particularly common utilities and structs e.g. Coords, various initial parsings of input_lines.
      - In particular, might change the standard input to be a &[Vec<String>] where the outer vec is split on double-line breaks, and the inner vec is split on single line breaks. This is a common format in AOC inputs, used to separate parts of the input.
    - Change default variable type for output of functions to i32.  I'm not sure I've ever seen a negative answer, but generally handling i32s is better (they allow subtraction!) through the code.  Or maybe to a string, so we can handle the odd occasion where a string is needed!
- year20
  - All solutions for 2020, in a more organised format.
  - Day 21 outputs in a weird way, due to having a string result for Part 2 (it prints the Part 2 string result, then the Part 1 answer, then a 'numerical' answer for Part 2 of 0).
  - Day 23 part 2 hits a stack overflow issue when run on my personal Windows PC but runs successfully in my work WSL.
- year21
  - Complete with the following caveats:
    - Day 13 has a visual text output which needs human parsing. I'm vaguly considering implementing something to read this.
    - Day 24 was a special day where the best way of solving it was very much through human reading and parsing of the input. The implemented code gets the correct answer for my input, but only in a reasonable time because I've set the starting point of the search bounds suitably close. With enlarged bounds it should get the correct answers for any input, but would take an *extremely* long time (potentially decades).
      - There's a couple of ways of implementing something smarter that I've seen mentioned online: from using knowledge of the structure of the provided code (base 26 stuff) to look for the relevant variables and calculate, to implementing a full on mathematical logic parser which can identify the constraints from entirely arbitrary code.  I'm not planning on doing any of these.
  - The following days take longer than 1s, which would be nice to improve on:
    - Day 18 (~ 1.2s)
    - Day 19 (~ 5.7s)
    - Day 22 (~17.4s)
    - Day 24 (~ 2.8s)
      - Although note this time is dependent on how I've set the limits; with the approach taken I could equally well cut it down to microseconds, and with a complete solution it'd take decades.
  - All the template thoughts for improvement apply to this codebase (apart from adding top level tests, which I've done on a day-by-day basis).
