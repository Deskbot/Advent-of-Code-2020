use std::collections::HashSet;
use std::fs;
use regex::Regex;

pub fn day04() {
    let file = fs::read_to_string("input/day04.txt")
        .expect("input not found");

    println!("Part 1: {}", part1(&file));
}

fn part1(file: &str) -> usize {
    let passports = split_into_passports(&file);

    let valid_passports = passports.iter()
        .map(|passport| validate_passport(passport));

    return valid_passports.filter(|&b| b).count();
}

fn split_into_passports(file: &str) -> Vec<&str> {
    return file.split("\n\n").collect();
}

fn split_passport_into_fields(passport: &str) -> Vec<&str> {
    let re = Regex::new("[\n ]").unwrap();
    return re.split(passport).collect();
}

fn validate_fields(fields: &Vec<&str>) -> bool {
    let need: HashSet<&'static str> = [
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid",
    ].iter().cloned().collect();

    for item in need {
        let found = fields.iter()
            .any(|field| field.starts_with(item));

        if found == false {
            return false;
        }
    }

    return true;
}

fn validate_passport(passport: &str) -> bool {
    let fields = split_passport_into_fields(passport);
    return validate_fields(&fields);
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn part1() {
        let file = fs::read_to_string("input/day04.txt")
        .expect("input not found");

        assert_eq!(super::part1(&file), 202);
    }

    #[test]
    fn part1_example() {
        assert_eq!(super::part1(EXAMPLE_INPUT), 2);
    }

    #[test]
    fn part1_1() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm";
        assert!(validate_passport(input));
    }

    #[test]
    fn part1_2() {
        let input = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929";
        assert!(!validate_passport(input));
    }

    #[test]
    fn part1_3() {
        let input = "hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm";
        assert!(validate_passport(input));
    }

    #[test]
    fn part1_4() {
        let input = "
hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        assert!(!validate_passport(input));
    }
}
