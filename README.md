# AdventOfCode

Repo for Advent of Code challenges.

I'm primarily doing these for fun and general experience with Rust, with a smattering of learning new stuff on the side.

## Contents:
- 2019
  - Most solutions for 2019, in a fairly scattershot arrangement (most of the puzzles involving the computer are in all all_days, other puzzles are their own cargo crates)
  - Would like, at some point, to go back and (a) clean this up and (b) complete the missing puzzles!
- template
  - Framework files for a new year.  Feel free to clone!  Details below in the "How to Use".
  - Thoughts for improvement:
    - Add bit to script to get the inputs automatically each day/when first run after the given day
    - Add some particularly common utilities and structs e.g. Coords.
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
  - This was done prior to my more recent Template updates, so uses a few different approaches/function signatures.

## How To Use The Template
Begin by copy-pasting the template directory into a new name (it should start with a letter).  You may want to delete the existing year directory with my answers and use those names!

Add your personal input (the contents of https://adventofcode.com/<YEAR>/day/<DAY>/input) to the relevant input file (e.g. year<YEAR>/inputs/<DAY>).

Add your code to the relevant src file (e.g. year<YEAR>/src/day<DAY>.rs)
- Recommend changing the top level function's input parameter to remove the leading underscore (it's there to stop Rust complaining when the functions are empty).
- The input is provided as a `&[Vec<String>]`. Your input lines are split into top level slice elements split on double-line breaks, with individual lines forming the Strings of a lower-level slice.
  - For days where the input doesn't have any double-line splits, you may want the first line of your code to be `let input_lines = input_lines[0]` for simplicity.
- The output required is a pair of Strings, which will be printed to terminal. In the vast majority of days, the result values are numbers, but occasionally strings are wanted!
- The test frameowrk in each file can also be used to put example cases from the puzzle page with the example answers inline, following the comments.

To run the code, simply run `cargo run <DAY>` from within your copied directory. Remember to use `--release` if you want to compare run-times!