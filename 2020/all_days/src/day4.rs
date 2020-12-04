use std::collections::HashMap;

pub fn day4(input_lines: &[String]) -> (u64, u64) {
    let passports: Vec<Passport> = input_lines[0]
        .split("\n\n")
        .map(|passport| make_passport(passport))
        .collect();
    // Create Passport object that
    (passports.iter().filter(|p| valid(p)).count() as u64, 0)
}

fn make_passport(input: &str) -> Passport {
    let mut hash: HashMap<&str, &str> = HashMap::new();
    let broken_string = input.split(|c| c == ' ' || c == '\n');

    for element in broken_string {
        let separated: Vec<&str> = element.split(':').collect();
        hash.insert(separated[0], separated[1]);
    }

    Passport {
        byr: hash.get("byr").map(|s| s.to_string()),
        iyr: hash.get("iyr").map(|s| s.to_string()),
        eyr: hash.get("eyr").map(|s| s.to_string()),
        hgt: hash.get("hgt").map(|s| s.to_string()),
        hcl: hash.get("hcl").map(|s| s.to_string()),
        ecl: hash.get("ecl").map(|s| s.to_string()),
        pid: hash.get("pid").map(|s| s.to_string()),
    }
}

fn valid(p: &Passport) -> bool {
    p.byr.is_some()
        && p.iyr.is_some()
        && p.eyr.is_some()
        && p.hgt.is_some()
        && p.hcl.is_some()
        && p.ecl.is_some()
        && p.pid.is_some()
}

struct Passport {
    byr: Option<String>, // (Birth year)
    iyr: Option<String>, // (Issue year)
    eyr: Option<String>, // (Expiration Year)
    hgt: Option<String>, // (Height)
    hcl: Option<String>, // (Hair Color)
    ecl: Option<String>, // (Eye Color)
    pid: Option<String>, // (Passport ID)
}
