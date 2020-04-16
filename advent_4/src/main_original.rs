use std::cmp::Ordering;

// This is the brute-force approach: check every number in range to see
// if it's valid.  That's programming, right?

// Slightly smarter way: increment to plausible values; e.g. when
// spotting that the tens digit goes up, skip the units digit to match it.

// Much smarter way: just construct valid passwords.  It's a bit complex in
// the 200,000s and 700,000s because of the bounds but the rest is fairly
// simple.

// Given part 2, the much smarter way might fall down somewhat.  The slightly
// smarter way could still work.

fn main() {

    let mut valid_count = 0;

    // This is somewhat inefficient.  Arguably we could do something
    // smarter with incrementing the number in ways that bypasses a
    // lot of obviously wrong elements.
    for password in 240920..789857 {
        if check_password(password) {valid_count += 1;};
    }

    println!("Total number of valid passwords in part 1 {}", valid_count);
    valid_count = 0;

    for password in 240920..789857 {
        if check_advanced_password(password) {valid_count += 1;};
    }

    println!("Total number of valid passwords in part 2 {}", valid_count);


}

fn check_password(test_password: i32) -> bool {
    // We'll only pass numbers in the range through to this test.
    // Need to split the number into a vector.
    // Could just use aritmetic but that is probably longer than
    // treating as a string and splitting...

    let mut test_vector: Vec<i32> = Vec::new();

    for digit_char in test_password.to_string().trim().chars() {
        let digit_string: i32 = digit_char.to_string().parse().expect("");
        test_vector.push(digit_string);
    }

    let mut previous_digit: i32 = 0;
    let mut found_pair = false;

    // Check that the digits are non-decreasing
    for digit in test_vector {
        match digit.cmp(&previous_digit) {
            Ordering::Less => return false,
            Ordering::Greater => {previous_digit = digit;},
            Ordering::Equal => {found_pair = true;},
        }
    }

    // We've checked non-decreasing. This is a valid-password iff we
    // found a pair.
    return found_pair;
}

fn check_advanced_password(test_password: i32) -> bool {
    // We'll only pass numbers in the range through to this test.
    // Need to split the number into a vector.
    // Could just use aritmetic but that is probably longer than
    // treating as a string and splitting...

    let mut test_vector: Vec<i32> = Vec::new();

    for digit_char in test_password.to_string().trim().chars() {
        let digit_string: i32 = digit_char.to_string().parse().expect("");
        test_vector.push(digit_string);
    }

    let mut previous_digit: i32 = 0;
    let mut consecutive_matching_digits = 1;
    let mut found_pair = false;

    // Check that the digits are non-decreasing
    for digit in test_vector {
        match digit.cmp(&previous_digit) {
            Ordering::Less => return false,
            Ordering::Greater => {
                if consecutive_matching_digits == 2 {
                    found_pair = true;
                }
                consecutive_matching_digits = 1;
                previous_digit = digit;
            },
            Ordering::Equal => {
                consecutive_matching_digits += 1;
            },
        }
    }

    if consecutive_matching_digits == 2 {
        found_pair = true;
    }

    // We've checked non-decreasing. This is a valid-password iff we
    // found a pair.
    return found_pair;
}
