// Assumed info:
// Options:
// - work out the angles, do trig.
// - do manual bashing
fn main() {

    let computer = all_days::define_computer("input/day19.txt");

    // Could try doing something more clever for Part 1
    // For example, work out the end points of the arc within the grid
    // then only check the sub-rectangle (or even less).
    // Or only check cells in a column starting from wherever the first 1 was in
    // a previous column, and stopping when we hit a 0.
    // But while the following is inefficient processing, it's a single command
    // solution including the print :D
    println!("Part 1 Answer: {}",
             (0..50)
             .map(|y| (0..50).map(|x|
                                  computer.clone_computer()
                                  .run_computer(&mut vec![y, x])[0])
                                  .filter(|&o| o == 1)
                                  .count())
             .sum::<usize>());


    let mut x = 100;
    let mut y = 0;
    let mut in_beam = computer.clone_computer().run_computer(&mut vec![y, x])[0] == 1;

    loop {
        // Go down until we find a cell in the beam.
        while !in_beam {
            y += 1;
            in_beam = computer.clone_computer().run_computer(&mut vec![y, x])[0] == 1;
        }

        // Go right until we're out of the beam again.
        while in_beam {
            x += 1;
            in_beam = computer.clone_computer().run_computer(&mut vec![y, x])[0] == 1;
        }

        // Check if the last found cell was valid.
        if computer.clone_computer().run_computer(&mut vec![y + 99, x - 100])[0] == 1 {
            break;
        }
    }

    // Last cell checked was one right of the upper right corner.
    println!("Part 2 Answer: {}", (x-100) * 10000 + y);
}
