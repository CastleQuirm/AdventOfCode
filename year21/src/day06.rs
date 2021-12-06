// Potential improvements:
// I guess the school could use an array rather than 9 variables? But otherwise this is pretty clean.
// Maybe I could do something more formally involving an iterator for the next()?

pub fn day06(input_lines: &[String]) -> (u64, u64) {
    let mut initial_population = [0_u64; 9];
    input_lines
        .first()
        .expect("Couldn't read first line")
        .split(',')
        .for_each(|c| {
            let i = c.parse::<usize>().expect("Couldn't parse number");
            initial_population[i] += 1;
        });
    let mut school = School {
        time_0: initial_population[0],
        time_1: initial_population[1],
        time_2: initial_population[2],
        time_3: initial_population[3],
        time_4: initial_population[4],
        time_5: initial_population[5],
        time_6: initial_population[6],
        time_7: initial_population[7],
        time_8: initial_population[8],
    };
    (0..80).for_each(|_| school = school.next());
    let part_1 = school.total();
    (80..256).for_each(|_| school = school.next());
    (part_1, school.total())
}

struct School {
    time_0: u64,
    time_1: u64,
    time_2: u64,
    time_3: u64,
    time_4: u64,
    time_5: u64,
    time_6: u64,
    time_7: u64,
    time_8: u64,
}

impl School {
    fn next(&self) -> Self {
        School {
            time_0: self.time_1,
            time_1: self.time_2,
            time_2: self.time_3,
            time_3: self.time_4,
            time_4: self.time_5,
            time_5: self.time_6,
            time_6: self.time_7 + self.time_0,
            time_7: self.time_8,
            time_8: self.time_0,
        }
    }
    fn total(&self) -> u64 {
        self.time_0
            + self.time_1
            + self.time_2
            + self.time_3
            + self.time_4
            + self.time_5
            + self.time_6
            + self.time_7
            + self.time_8
    }
}
