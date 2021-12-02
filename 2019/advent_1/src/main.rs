use std::fs;

fn main() {
    let fuel_total = fuel_for_components();

// First pass, wrong due to misreading requirements (specifically, this
// worked out the fuel for every component, then the fuel for that total
// fuel, and the weight for that total).  Kept for posterity as a comparison
// to the method I use in fuel_for_component().
    // println!("Fuel for just the components: {}", base_fuel);

    // let mut fuel_array: Vec<i32> = vec![base_fuel];

    // loop {
    //     let _further_fuel = fuel_for_thing(fuel_array[fuel_array.len() - 1]);
    //     if _further_fuel > 0 {
    //         fuel_array.push(_further_fuel);
    //     } else {
    //         break;
    //     }
    // };

    // let mut fuel_total = 0;

    // for _fuel_entry in fuel_array {
    //     fuel_total += _fuel_entry;
    // };

    println!("Total fuel: {}", fuel_total);

}

fn fuel_for_components() -> i32 {

    let module_mass_list = fs::read_to_string("module_list.txt")
        .expect("Failed to read file");

    let mut fuel_total = 0;

    for _module_mass in module_mass_list.lines() {
        let _module_mass: i32 = match _module_mass.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let _fuel_value = fuel_for_component(_module_mass);
        fuel_total += _fuel_value;
    }

    return fuel_total;
}

// This function covers Part 2 of Advent 1.  For Part 1, just have this
// call fuel_for_thing() directly rather than doing any work.
fn fuel_for_component(module_mass: i32) -> i32 {
    let mut current_unfueled_element = module_mass;
    let mut module_fuel = 0;
    loop {
        let new_fuel = fuel_for_thing(current_unfueled_element);
        if new_fuel <= 0 {
            break;
        }
        module_fuel += new_fuel;
        current_unfueled_element = new_fuel;
    }
    return module_fuel;
}

fn fuel_for_thing(thing_mass: i32) -> i32 {
    let _calculated_value = (thing_mass / 3) - 2;
    if _calculated_value < 0 {
        0
    } else {
        _calculated_value
    }
}
