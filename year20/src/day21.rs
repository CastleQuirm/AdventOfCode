// Possible improvements:
// 1) Actually solve Part 2!  But it has a string result rather than a u64, so needs some small infrastructure changes as well as the last step.
// 2) See if I can remove the muts and while loop.  Break up some of the main function while I'm at it.
// 3) Better naming - some of the latter parts of the main function get iffy between "foods" and "ingredients"
// 4) Make Ingredient and Allergen into types so I can reference them directly
// 5) Make some of the more general functions below into trait-generic functions - things like the all_allergens, which could be union_fields<T>(input: &[T], access_function: f(T)->S) -> HashSet<S>

use std::collections::{HashMap, HashSet};

pub fn day21(input_lines: &[String]) -> (u64, u64) {
    // Read input
    let all_foods = input_lines
        .iter()
        .map(|line| Food::new(line))
        .collect::<Vec<Food>>();
    let mut undiscovered_allergens = all_allergens(&all_foods);
    let mut fixed_ingredients: HashMap<String, String> = HashMap::new();
    while !(&undiscovered_allergens.is_empty()) {
        let causing_foods =
            determine_causes(&undiscovered_allergens, &all_foods, &fixed_ingredients);
        let discovered_allergen = undiscovered_allergens
            .iter()
            .find(|&allergen| causing_foods.get(allergen).expect("unknown allergen").len() == 1)
            .expect("No single derivable allergen")
            .clone();
        let discovered_food = causing_foods
            .get(&discovered_allergen)
            .unwrap()
            .iter()
            .next()
            .unwrap();
        undiscovered_allergens.remove(&discovered_allergen);
        fixed_ingredients.insert(discovered_allergen.clone(), discovered_food.clone());
        // println!("{} contains {}", discovered_food, discovered_allergen);
    }

    // We now have a list of all ingredients that have an allergen.
    let all_allergenic_ingredients = fixed_ingredients
        .values()
        .map(std::string::ToString::to_string)
        .collect::<HashSet<String>>();
    let part1_answer = all_foods
        .iter()
        .map(|food| {
            food.ingredients
                .difference(&all_allergenic_ingredients)
                .count() as u64
        })
        .sum::<u64>();

    let mut dangerous_allergens = fixed_ingredients
        .keys()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>();
    dangerous_allergens.sort_unstable();
    let dangerous_ingredients = dangerous_allergens.iter().map(|allergen| {
        fixed_ingredients
            .get(allergen)
            .expect("didn't recognise allergen")
            .to_string()
    });
    let part2_answer = dangerous_ingredients
        .reduce(|mut list, ingredient| {
            list.push(',');
            list.push_str(&ingredient);
            list
        })
        .expect("No dangerous ingredients?");
    println!("Part 2 answer: {}", part2_answer);

    // For each allergen A, find the set of possible ingredients {i} that could be the source by finding every Food {f} that contains A and taking the intersection of all f.ingredients
    // Then find such an allergen A that contains only one element I outside of ingredients_with_allergens
    (part1_answer, 0)
}

// Gets a HashSet of all unique Allergens present in a selection of Foods
fn all_allergens(all_foods: &[Food]) -> HashSet<String> {
    all_foods.iter().fold(HashSet::new(), |collection, food| {
        collection
            .union(&food.allergens)
            .cloned()
            .collect::<HashSet<String>>()
    })
}

// Creates a HashMap of allergens => HashSet<causing foods>
fn determine_causes(
    undiscovered_allergens: &HashSet<String>,
    all_foods: &[Food],
    fixed_ingredients: &HashMap<String, String>,
) -> HashMap<String, HashSet<String>> {
    undiscovered_allergens
        .iter()
        .fold(HashMap::new(), |mut map, allergen| {
            map.insert(
                allergen.clone(),
                find_possible_ingredient(allergen, all_foods, fixed_ingredients),
            );
            map
        })
}

// Gets the number of possible ingredients which might contain each allergen for a HashSet of allergens and the set of Foods.
// fn possibe_ings(undiscovered_allergens: &HashSet<String>, all_foods: &[Food]) -> Vec<usize> {
//     undiscovered_allergens
//         .iter()
//         .map(|allergen| find_possible_ingredient(allergen, all_foods).len())
//         .collect::<Vec<usize>>()
// }

fn find_possible_ingredient(
    allergen: &str,
    all_foods: &[Food],
    fixed_ingredients: &HashMap<String, String>,
) -> HashSet<String> {
    let all_allergenic_ingredients = fixed_ingredients
        .values()
        .map(std::string::ToString::to_string)
        .collect::<HashSet<String>>();
    shared_ingredient(
        &all_foods
            .iter()
            .filter(|food| food.allergens.contains(allergen))
            .map(|food| {
                food.ingredients
                    .difference(&all_allergenic_ingredients)
                    .clone()
                    .map(std::string::ToString::to_string)
                    .collect::<HashSet<String>>()
            })
            .collect::<Vec<HashSet<String>>>(),
    )
}

fn shared_ingredient(ingredients_lists: &[HashSet<String>]) -> HashSet<String> {
    ingredients_lists.iter().fold(
        ingredients_lists.first().unwrap().clone(),
        |intersection, list| {
            intersection
                .intersection(list)
                .cloned()
                .collect::<HashSet<String>>()
        },
    )
}

#[derive(Debug)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}
impl Food {
    fn new(line: &str) -> Food {
        let first_split = line.split(" (contains ").collect::<Vec<&str>>();
        Food {
            ingredients: first_split[0]
                .split(' ')
                .map(std::string::ToString::to_string)
                .collect::<HashSet<String>>(),
            allergens: first_split[1]
                .split(')')
                .next()
                .expect("No ) found")
                .split(", ")
                .map(std::string::ToString::to_string)
                .collect::<HashSet<String>>(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day21;

    #[test]
    fn day21_part1_example() {
        let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day21(&input), (5, 0));
    }
}
