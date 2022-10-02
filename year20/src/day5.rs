// Potential improvements:
// 1: Something more computationally efficient for part 2.  Don't search the entire list of boarding passes for every number!  E.g. just sort, then walk numbers until we skip one.
// 2: Something more elegant in boarding_seat_as_uid?  Neater way to split the parts of the string?  Reuse code for the two halves somehow?  Make more functional?
// 3: Something neater in finding the seat for Part 2?  Separate function?  Not swapping back and forth between Vecs and Iters?

pub fn day5(input_lines: &[String]) -> (u64, u64) {
    let taken_seats = input_lines
        .iter()
        .map(|line| boarding_seat_as_uid(line))
        .collect::<Vec<u64>>();
    let min_seat = taken_seats.iter().min().expect("Inputs were empty?");
    let max_seat = taken_seats.iter().max().expect("Inputs were empty?");
    let potential_seats = (*min_seat..*max_seat)
        .filter(|seat| !taken_seats.contains(seat))
        .collect::<Vec<u64>>();
    assert!(potential_seats.len() == 1);

    (*max_seat, potential_seats[0])
}

fn boarding_seat_as_uid(seat: &str) -> u64 {
    let row_binary = &seat[0..7].replace('F', "0").replace('B', "1");
    let row_num = isize::from_str_radix(row_binary, 2).unwrap() as u64;

    let col_binary = &seat[7..].replace('L', "0").replace('R', "1");
    let col_num = isize::from_str_radix(col_binary, 2).unwrap() as u64;

    row_num * 8 + col_num
}
