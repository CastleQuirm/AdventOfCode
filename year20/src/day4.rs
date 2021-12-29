// Possible improvements
// 1: Make this more robust.  There's a bunch of inputs that would cause this to crash e.g. if any of BYR, IYR or EYR weren't a number.
// 2: Make more functional, in particular hgt_valid and hcl_valid
// 3: Maybe make the parameter values better tracked rather than magic values in the middle of the code
// 4: Break out "valid" to have subfunctions.
// 5: Neaten up all the String/&str transformations etc
// 6: Improve string consumption e.g. look into Nom.

use std::collections::HashMap;

pub fn day4(input_lines: &[String]) -> (u64, u64) {
    let mut passports: Vec<Passport> = Vec::new();
    let mut passport_lines: Vec<String> = Vec::new();
    for line in input_lines {
        if line.is_empty() {
            passports.push(Passport::new(&passport_lines));
            passport_lines = Vec::new();
        } else {
            passport_lines.push(line.clone());
        }
    }
    passports.push(Passport::new(&passport_lines));

    (
        passports.iter().filter(|p| present(p)).count() as u64,
        passports.iter().filter(|p| valid(p)).count() as u64,
    )
}

fn present(p: &Passport) -> bool {
    p.byr.is_some()
        && p.iyr.is_some()
        && p.eyr.is_some()
        && p.hgt.is_some()
        && p.hcl.is_some()
        && p.ecl.is_some()
        && p.pid.is_some()
}

fn valid(p: &Passport) -> bool {
    let byr_valid = (1920..2003).contains(&p.byr.unwrap_or(0));
    let iyr_valid = (2010..2021).contains(&p.iyr.unwrap_or(0));
    let eyr_valid = (2020..2031).contains(&p.eyr.unwrap_or(0));
    let hgt_valid = if p.hgt.is_some() {
        let hgt = p.hgt.as_ref().expect("");
        if hgt.len() < 4 {
            false
        } else {
            let num = &hgt[0..hgt.len() - 2].parse().expect("Can't parse height");
            match &hgt[hgt.len() - 2..] {
                "cm" => (150..194).contains(num),
                "in" => (59..77).contains(num),
                _ => false,
            }
        }
    } else {
        false
    };
    let hcl_valid = if p.hcl.is_some() {
        let hcl = p.hcl.as_ref().expect("");
        hcl.len() == 7
            && hcl.starts_with('#')
            && hcl.chars().filter(|&c| is_hex_digit(c)).count() == 6
    } else {
        false
    };
    let ecl_valid = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .map(|s| s.to_string())
        .any(|x| &x == p.ecl.as_ref().unwrap_or(&"".to_string()));
    let pid_valid = if p.pid.is_some() {
        let pid_unwrap: &String = p.pid.as_ref().unwrap();
        pid_unwrap.len() == 9 && pid_unwrap.parse::<i32>().is_ok()
    } else {
        false
    };

    byr_valid && iyr_valid && eyr_valid && hgt_valid && hcl_valid && ecl_valid && pid_valid
}

fn is_hex_digit(c: char) -> bool {
    match c.to_string().parse::<usize>() {
        Ok(_) => true,
        Err(_) => c == 'a' || c == 'b' || c == 'c' || c == 'd' || c == 'e' || c == 'f',
    }
}

#[derive(Debug)]
struct Passport {
    byr: Option<usize>,  // (Birth year)
    iyr: Option<usize>,  // (Issue year)
    eyr: Option<usize>,  // (Expiration Year)
    hgt: Option<String>, // (Height)
    hcl: Option<String>, // (Hair Color)
    ecl: Option<String>, // (Eye Color)
    pid: Option<String>, // (Passport ID)
}
impl Passport {
    fn new(input_lines: &[String]) -> Self {
        let mut hash: HashMap<&str, &str> = HashMap::new();
    
        let broken_string = input_lines.iter().flat_map(|line| line.split(|c| c == ' ')).collect::<Vec<&str>>();

        for element in broken_string {
            let separated: Vec<&str> = element.split(':').collect();
            hash.insert(separated[0], separated[1]);
        }
    
        Self {
            byr: hash
                .get("byr")
                .map(|s| s.to_string().parse().expect("BYR Not a number")),
            iyr: hash
                .get("iyr")
                .map(|s| s.to_string().parse().expect("IYR Not a number")),
            eyr: hash
                .get("eyr")
                .map(|s| s.to_string().parse().expect("EYR Not a number")),
            hgt: hash.get("hgt").map(|s| s.to_string()),
            hcl: hash.get("hcl").map(|s| s.to_string()),
            ecl: hash.get("ecl").map(|s| s.to_string()),
            pid: hash.get("pid").map(|s| s.to_string()),
        }
    }
}