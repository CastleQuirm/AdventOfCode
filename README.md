# AdventOfCode

Repo for Advent of Code challenges.

I'm primarily doing these for fun and general experience with Rust, with a smattering of learning new stuff on the side.

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
