// Possible improvements:
// 1) High level structural: there must be nicer ways to do this than with mutable lists and cloning all over the place?
// 2) High level: is there a better way to deal with the existing gamestates than a potentially huge HashSet?
// 3) More details: commonise the code for combat and recursive_combat.
// 4) Tidy up the read_input (and commonise the two players' deck-reading code.)

use std::collections::HashSet;

pub fn day22(input_lines: &[String]) -> (u64, u64) {
    let (mut p1, mut p2) = read_input(input_lines);

    (
        combat(&mut p1.clone(), &mut p2.clone()),
        recursive_combat(&mut p1, &mut p2).1,
    )
}

fn combat(p1: &mut Vec<usize>, p2: &mut Vec<usize>) -> u64 {
    while !p1.is_empty() && !p2.is_empty() {
        let x = p1.remove(0);
        let y = p2.remove(0);
        assert!(x != y);
        if x > y {
            p1.push(x);
            p1.push(y);
        } else {
            p2.push(y);
            p2.push(x);
        }
    }

    if p1.is_empty() {
        calculate_score(&p2)
    } else {
        calculate_score(&p1)
    }
}

// First return value is true if p1 wins, false if p2 wins.
fn recursive_combat(p1: &mut Vec<usize>, p2: &mut Vec<usize>) -> (bool, u64) {
    let mut gamestates: HashSet<(Vec<usize>, Vec<usize>)> = HashSet::new();
    while !p1.is_empty() && !p2.is_empty() && gamestates.insert((p1.clone(), p2.clone())) {
        // How do we track revisited states?
        let x = p1.remove(0);
        let y = p2.remove(0);
        assert!(x != y);

        let p1_wins = if x <= p1.len() && y <= p2.len() {
            let mut p1_subdeck: Vec<usize> = vec![0; x];
            p1_subdeck.clone_from_slice(&p1[0..x]);
            let mut p2_subdeck: Vec<usize> = vec![0; y];
            p2_subdeck.clone_from_slice(&p2[0..y]);
            recursive_combat(&mut p1_subdeck, &mut p2_subdeck).0
        } else {
            x > y
        };

        if p1_wins {
            p1.push(x);
            p1.push(y);
        } else {
            p2.push(y);
            p2.push(x);
        }
    }

    if p1.is_empty() {
        (false, calculate_score(&p2))
    } else {
        (true, calculate_score(&p1))
    }
}

fn read_input(input_lines: &[String]) -> (Vec<usize>, Vec<usize>) {
    let decks = input_lines[0].split("\n\n").collect::<Vec<&str>>();
    assert_eq!(decks.len(), 2);
    let mut p1 = decks[0].lines().collect::<Vec<&str>>();
    let mut p2 = decks[1].lines().collect::<Vec<&str>>();
    let player1_str = p1.remove(0);
    let player2_str = p2.remove(0);
    assert_eq!(player1_str, "Player 1:");
    assert_eq!(player2_str, "Player 2:");

    (
        p1.iter()
            .map(|line| {
                line.parse::<usize>()
                    .expect("Couldn't parse line as number")
            })
            .collect::<Vec<usize>>(),
        p2.iter()
            .map(|line| {
                line.parse::<usize>()
                    .expect("Couldn't parse line as number")
            })
            .collect::<Vec<usize>>(),
    )
}

fn calculate_score(deck: &[usize]) -> u64 {
    (0..deck.len()).fold(0, |score, i| score + (deck.len() - i) * deck[i]) as u64
}

#[cfg(test)]
mod tests {
    use super::day22;

    #[test]
    fn day22_example() {
        let input = vec!["Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"
        .to_string()];
        assert_eq!(day22(&input), (306, 291));
    }
}
