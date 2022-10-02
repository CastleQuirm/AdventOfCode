// Possible improvements: so many!
// 1: Make it more functional (how?) (remove the extensive use of muts, whiles and fors, but...what does the result look like?)
// 2: Make it faster (current version takes > 1s) (how?)
// 3: Would approach (b) be nicer?  For either part 1 or 2?
// 4: Add types for the Hashes, which is more readable, more useful and approaches Clippy better.
// 5: Rename the multiple similarly named things.
// 6: Other?

use array_tool::vec::Union;
use std::collections::HashMap;

pub fn day7(input_lines: &[String]) -> (u64, u64) {
    // Thoughts on approaches
    // a) Big structural approach - hash-maps both ways. <--- DONE THIS
    // b) Direct Part 1 approach: remove rule "shiny gold bags contain...", then set up mut counter=0 and iteratively call function:
    // (unused_rules: &[String], possibly_colours: Vec<colours>) -> (still_unused_rules: &[String], found_rules: Vec<colours>)
    // Adding the length of found_rules to counter each time, and passing the two outputs direct in as new inputs, until found_rules.len() == 0.

    let (contained, containing) = read_input_into_hashes(input_lines);

    let mut candidate_holder: Vec<String> = vec!["shiny gold".to_string()];
    let mut all_holding_colours: Vec<String> = Vec::new();

    while !candidate_holder.is_empty() {
        let new_colour = candidate_holder.drain(0..1).collect::<Vec<String>>();
        let possible_colour = new_colour
            .get(0)
            .expect("Didn't find anything in candidate holder");
        let holding_colours = containing.get(possible_colour);
        if let Some(colour) = holding_colours {
            all_holding_colours = all_holding_colours.union(colour.clone());
            candidate_holder = candidate_holder.union(colour.clone());
        }
    }

    let mut contents: HashMap<String, usize> =
        [("shiny gold".to_string(), 1)].iter().cloned().collect();
    let mut bag_count = 0;

    // Bit of a mess here!
    while !contents.is_empty() {
        let new_colour = contents
            .keys()
            .next()
            .expect("Contents weren't empty")
            .clone(); // "shiny gold"
        let number_of_colour = contents
            .remove(&new_colour)
            .expect("Colour wasn't a key after all"); // 1
        let holding_colours = contained
            .get(&new_colour)
            .expect("Didn't define the contents")
            .clone(); // Contents of shiny gold
        bag_count += number_of_colour;
        for (amount, inner_bag) in holding_colours {
            let existing_count = *contents.get(&inner_bag).unwrap_or(&0);
            contents.insert(inner_bag, amount * number_of_colour + existing_count);
        }
    }

    (all_holding_colours.len() as u64, (bag_count - 1) as u64)
}

fn read_input_into_hashes(
    input_lines: &[String],
) -> (
    HashMap<String, Vec<(usize, String)>>,
    HashMap<String, Vec<String>>,
) {
    let mut contained_map: HashMap<String, Vec<(usize, String)>> = HashMap::new();
    let mut containing_map: HashMap<String, Vec<String>> = HashMap::new();
    for line in input_lines {
        let (container, contained_list) = read_line_into_hashes(line);
        contained_map.insert(container.clone(), contained_list.clone());
        for (_num, colour) in contained_list {
            let existing_entry = containing_map.get(&colour).unwrap_or(&Vec::new()).clone();
            containing_map.insert(colour, vec![container.clone()].union(existing_entry));
        }
    }
    (contained_map, containing_map)
}

fn read_line_into_hashes(line: &str) -> (String, Vec<(usize, String)>) {
    let read_text = line.split(" bags contain ").collect::<Vec<&str>>();
    assert!(read_text.len() == 2);
    let outside_bag = read_text[0];
    let inside_bags = if read_text[1] != "no other bags." {
        read_text[1]
            .split(", ")
            .map(read_content_into_pair)
            .collect::<Vec<(usize, String)>>() // elements end in " bag[s]?[.]?"
    } else {
        Vec::new()
    };
    (outside_bag.to_string(), inside_bags)
}

// Input is always "[digit] [two-word colour] bag[s]?[.]?"...unless it says "no other bags"
fn read_content_into_pair(contents: &str) -> (usize, String) {
    let count = contents[0..1].parse().expect("Couldn't parse count");
    let last_space = contents.rfind(' ').expect("Couldn't find a space");
    let colour = contents[2..last_space].to_string();
    (count, colour)
}
