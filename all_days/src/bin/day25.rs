// Setup: we have a text adventure with a layout of rooms and a bunch of items
// We need to pick up a specific unknown subset of them and go to a specific location, then...?
// Some items are traps - they'll cause the computer to crash if taken, in various tricky ways
// Steps:
// - 1 (IDEAL): Have the program explore the map and find the items, locations, and Security Checkpoint
// - 2 (IDEAL): Have the program work out which items are traps.
// - 3: Trial and error the set of items needed.
//      - Pick up an item and try the checkpoint.  If too light, pick up more items and retry until just right or too heavy.
//      - If too heavy, restart and pick up the same items up until the too heavy point.  Try further items on the list.
//      - If list ends while still too light (having bailed on combinations that were too heavy) restart and take every item except the last accepted one
// - 3a: Alternative: try every combination by setting the legit items as bit flags and testing all combinations.
//      - Went with this one
//      - Didn't notice that you CAN drop items, which would have made the running a fair bit faster.
//      - (Correct approach: gather all items over the ship, go to security checkpoint, just do pick ups and drops there)
// - 4 (IDEAL): Parse the output to present the value in the multi-run option, rather than manually checking the blurb when it succeeds.

// use std::io;
use std::collections::HashMap;
use all_days::Computer;

fn main() {
    let mut orig_computer = all_days::define_computer("input/day25.txt");

    display_screen(&mut orig_computer.run_computer());

//* SCC How we found it
    // for i in 0..256 {
    //     let mut computer = orig_computer.clone_computer();
    //     collect_items(&mut computer, i);
    //     print!("Test i {}: ", i);
    //     check_weight(&mut computer);
    // }

    (0u32..8u32)
        .filter(|&exp| (226 / (2usize.pow(exp))) % 2 == 1)
        .for_each(|exp| collect_item(&mut orig_computer, 2usize.pow(exp)));
    check_weight(&mut orig_computer);

    // loop {
    //     let mut input = String::new();

    //     io::stdin().read_line(&mut input)
    //         .expect("Failed to read line");

    //     // match input {
    //     //     "n" | "north" => "north",
    //     //     "s" | "south" => "south",
    //     //     "e" | "east" => "east",
    //     //     "w" | "west" => "west",
    //     //     "t"
    //     // }

    //     display_screen(&computer.provide_ascii_input(&input))
    // }
}

fn collect_item(computer: &mut Computer, index: usize) {
    let item_flag: HashMap<usize, String> = [
        (1, "boulder".to_string()),
        (2, "asterisk".to_string()),
        (4, "food ration".to_string()),
        (8, "candy cane".to_string()),
        (16, "loom".to_string()),
        (32, "mutex".to_string()),
        (64, "mug".to_string()),
        (128, "prime number".to_string()),
    ].iter().cloned().collect();

    let get_item: HashMap<String, String> = [
        ("boulder".to_string(), "south\ntake boulder\nnorth\n".to_string()),
        ("asterisk".to_string(), "south\nwest\ntake asterisk\neast\nnorth\n".to_string()),
        ("food ration".to_string(), "south\neast\ntake food ration\nwest\nnorth\n".to_string()),
        ("candy cane".to_string(), "east\ntake candy cane\nwest\n".to_string()),
        ("loom".to_string(), "east\neast\nnorth\ntake loom\nsouth\nwest\nwest\n".to_string()),
        ("mutex".to_string(), "east\nnorth\nnorth\ntake mutex\nsouth\nsouth\nwest\n".to_string()),
        ("mug".to_string(), "east\nnorth\neast\nnorth\ntake mug\nsouth\nwest\nsouth\nwest\n".to_string()),
        ("prime number".to_string(), "east\nnorth\nnorth\nnorth\ntake prime number\nsouth\nsouth\nsouth\nwest\n".to_string()),
        ].iter().cloned().collect();

    display_screen(&computer.provide_ascii_input(&get_item.get(item_flag.get(&(index)).expect("indexing wrong")).expect("")));
    always_display_screen(&computer.provide_ascii_input("inv\n"));
}

fn check_weight(computer: &mut Computer) {
    let test_weight = String::from("east\neast\neast\nsouth\neast\neast\nnorth\n");
    let output_vec = computer.provide_ascii_input(&test_weight);
    let output_string: String = output_vec.clone().iter().filter(|&i| *i <= 255).map(|&i| i as u8 as char).collect();
    display_screen(&output_vec);

    if output_string.contains("heavier") {
        println!("Robot is too light!");
        always_display_screen(&computer.provide_ascii_input("inv\n"));
    } else if output_string.contains("lighter") {
        println!("Robot is too heavy!");
        always_display_screen(&computer.provide_ascii_input("inv\n"));
    } else {
        println!("Robot is just right!");
        always_display_screen(&computer.provide_ascii_input("inv\n"));
        panic!("ooooh");
    }

    computer.provide_ascii_input("west\nwest\nnorth\nwest\nwest\nwest\n");
}

fn display_screen(output_vec: &Vec<i64>) {
    let output_string: String = output_vec.iter().filter(|&i| *i <= 255).map(|&i| i as u8 as char).collect();
    let output_lines: Vec<&str> = output_string.split("\n").collect();

    for line in output_lines {
        println!("{:?}", line);
    }

    let other_output: Vec<&i64> = output_vec.iter().filter(|&i| *i > 255).collect();
    if !other_output.is_empty() {
        println!("{}", other_output[0]);
    }
}

// Debug function for when I want to hide the verbose output!
fn always_display_screen(output_vec: &Vec<i64>) {
    let output_string: String = output_vec.iter().filter(|&i| *i <= 255).map(|&i| i as u8 as char).collect();
    let output_lines: Vec<&str> = output_string.split("\n").collect();

    for line in output_lines {
        println!("{:?}", line);
    }

    let other_output: Vec<&i64> = output_vec.iter().filter(|&i| *i > 255).collect();
    if !other_output.is_empty() {
        println!("{}", other_output[0]);
    }
}
