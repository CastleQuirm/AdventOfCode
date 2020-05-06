// Note: Multiplication is good!
// Assumption: any given product can only be made one way
// Assumption: any given formula only produces one type of product

use std::fs;
use std::collections::HashMap;

fn main() {

    // Parse the input into a hash-map ->
    // Key: type of output
    // Val: (amount in output, set of inputs as Ingredients)
    let mut formulae: HashMap<String, (i64, Vec<Ingredient>)> = HashMap::new();

    let string = fs::read_to_string("input/day14.txt").expect("Failed to read file");
    for line in string.lines() {
        let sides: Vec<&str> = line.trim().split(" => ").collect();
        assert!(sides.len() == 2);
        let components: Vec<&str> = sides[0].split(", ").collect();
        let result: Vec<&str> = sides[1].split(" ").collect();
        assert!(result.len() == 2);

        let mut ingredient_list: Vec<Ingredient> = Vec::new();
        for component in components {
            let split_component: Vec<&str> = component.split(" ").collect();
            let amount = split_component[0].parse().expect("");
            ingredient_list.push(make_ingredient(&(&split_component[1].to_string(), &amount)));
        }

        formulae.insert(result[1].to_string(),
                        (result[0].parse().expect(""),
                         ingredient_list));
    }

    // Define what we have in stock.  Negative values are what we need to make.
    // For part 1 we need to make 1 FUEL
    let mut stock: HashMap<String, i64> = [("FUEL".to_string(), -1)]
        .iter().cloned().collect();
    produce_fuel(&mut stock, &formulae);
    let one_fuel_ore = *stock.get("ORE").expect("");
    println!("Part 1 Answer: {}", one_fuel_ore);

    // For part 2 we have 1 trillion ORE.  We want to work out how much FUEL we
    // can make; this will be at least 1 trillion divided by the part 1 answer,
    // and probably higher from the excess materials we create.  Start from the
    // lower bound and re-run produce_fuel for ever increasing amounts of FUEL
    // until we need more ORE than we can take.

    // Could slightly cut things down by adding multiple FUEL after the first,
    // if the amount of ORE we've got spare is a multiple of the first answer.
    // SCC Update: this is actually mandatory to make it run in sensible time!

    let max_ore: i64 = 1_000_000_000_000;
    let min_fuel = max_ore / one_fuel_ore;

    let mut fuel_produced = min_fuel;

    stock = [("FUEL".to_string(), -min_fuel),
             ("ORE".to_string(), 0)].iter().cloned().collect();
    produce_fuel(&mut stock, &formulae);

    while *stock.get("ORE").expect("") < max_ore {
        let spare_ore = max_ore - *stock.get("ORE").expect("");
        let jump_fuel = spare_ore / one_fuel_ore;
        let old_amount = stock.remove("FUEL").expect("");
        stock.insert(String::from("FUEL"), old_amount - jump_fuel);
        produce_fuel(&mut stock, &formulae);
        fuel_produced += jump_fuel;
        assert!(*stock.get("ORE").expect("") < max_ore);

        let new_amount = stock.remove("FUEL").expect("");
        stock.insert(String::from("FUEL"), new_amount - 1);
        produce_fuel(&mut stock, &formulae);
        fuel_produced += 1;
    }

    println!("Part 2 Answer: {}", fuel_produced - 1);
}

struct Ingredient {
    component: String,
    amount: i64,
}

fn make_ingredient(definition: &(&String, &i64)) -> Ingredient {
    Ingredient {
        component: definition.0.to_string(),
        amount: *definition.1,
    }
}

fn produce_fuel(stock: &mut HashMap<String, i64>,
                formulae: &HashMap<String, (i64, Vec<Ingredient>)>) {
    // Keep going while we're missing items
    while stock.values().filter(|&&quant| quant < 0i64).count() > 0 {
        // Pick an item we're missing and make it!
        let product: Ingredient =
            make_ingredient(&stock
                            .iter()
                            .filter(|(_thing, amount)| **amount < 0i64)
                            .next()
                            .expect(""));

        // Get the formula for making the missing thing
        let formula: &(i64, Vec<Ingredient>) = formulae
            .get(&product.component)
            .expect("Don't know how to make this");

        // Process the formula until we have enough of the thing we're trying to make
        let amount_reqd = stock.get(&product.component).expect("").abs();
        let formula_mult =
            match amount_reqd % formula.0 {
                0 => amount_reqd / formula.0,
                _ => amount_reqd / formula.0 + 1,
            };

        let old_amount = stock.remove(&product.component).expect("");
        stock.insert(String::from(&product.component), old_amount + formula.0 * formula_mult);

        // Update stock levels for each ingredient
        for component in &formula.1 {
            let multiplier =
                if component.component == "ORE" {
                    formula_mult
                } else {
                    -formula_mult
                };
            match stock.remove_entry(&component.component) {
                Some((name, quant)) => stock.insert(String::from(&name), quant + multiplier * component.amount),
                None => stock.insert(String::from(&component.component), multiplier * component.amount),
            };
        }
    }
}
