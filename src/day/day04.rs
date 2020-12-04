use std::collections::HashSet;
use std::fs;
use regex::Regex;
use substring::Substring;

pub fn day04() {
    let file = fs::read_to_string("input/day04.txt")
        .expect("input not found");

    println!("Part 1: {}", part1(&file));
    println!("Part 2: {}", part2(&file));
}

fn part1(file: &str) -> usize {
    let passports = split_into_passports(&file);

    let valid_passports = passports.iter()
        .map(|passport| validate_passport_part1(passport));

    return valid_passports.filter(|&b| b).count();
}

fn part2(file: &str) -> usize {
    let passports = split_into_passports(&file);

    let valid_passports = passports.iter()
        .map(|passport| validate_passport_part2(passport));

    return valid_passports.filter(|&b| b).count();
}

fn split_into_passports(file: &str) -> Vec<&str> {
    return file.split("\n\n").collect();
}

fn split_passport_into_fields(passport: &str) -> Vec<&str> {
    let re = Regex::new("[\n ]").unwrap();
    return re.split(passport).collect();
}

fn validate_field_presence(fields: &Vec<&str>) -> bool {
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

fn validate_field(field: &str) -> bool {
    let (key, val) = split_field(field);

    match key {
        "byr" => validate_byr(val),
        "iyr" => validate_iyr(val),
        "eyr" => validate_eyr(val),
        "hgt" => validate_hgt(val),
        "hcl" => validate_hcl(val),
        "ecl" => validate_ecl(val),
        "pid" => validate_pid(val),
        "cid" => true,
        _ => panic!("poo")
    }
}

fn validate_fields(fields: &Vec<&str>) -> bool {
    fields.iter()
        .map(|field| validate_field(field))
        .all(|b| b)

}

fn split_field(field: &str) -> (&str, &str) {
    let mut itr = field.split(":");
    return (itr.next().unwrap(), itr.next().unwrap());
}

fn validate_byr(val: &str) -> bool {
    let year = val.parse::<i32>().unwrap();
    return year >= 1920 && year <= 2002;
}

fn validate_iyr(val: &str) -> bool {
    let year = val.parse::<i32>().unwrap();
    return year >= 2010 && year <= 2020;
}

fn validate_eyr(val: &str) -> bool {
    let year = val.parse::<i32>().unwrap();
    return year >= 2020 && year <= 2030;
}

fn validate_hgt(val: &str) -> bool {
    let cm = val.ends_with("cm");
    let inch = val.ends_with("in");

    if cm || inch {
        let height = val.substring(0, val.len() - 2).parse::<i32>().unwrap();

        if cm {
            return height >= 150 && height <= 193;
        } else {
            // inches
            return height >= 59 && height <= 76;
        }
    }

    return false;
}

fn validate_hcl(val: &str) -> bool {
    let re = Regex::new("#[0-9a-f]{6}").unwrap();
    return re.is_match(val);
}

fn validate_ecl(val: &str) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .any(|&valid_val| valid_val == val)
}

fn validate_pid(val: &str) -> bool {
    let re = Regex::new("^[0-9]{9}$").unwrap();
    return re.is_match(val);
}

fn validate_passport_part1(passport: &str) -> bool {
    let fields = split_passport_into_fields(passport);
    return validate_field_presence(&fields);
}

fn validate_passport_part2(passport: &str) -> bool {
    let fields = split_passport_into_fields(passport);
    return validate_field_presence(&fields) && validate_fields(&fields);
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
        assert!(validate_passport_part1(input));
    }

    #[test]
    fn part1_2() {
        let input = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929";
        assert!(!validate_passport_part1(input));
    }

    #[test]
    fn part1_3() {
        let input = "hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm";
        assert!(validate_passport_part1(input));
    }

    #[test]
    fn part1_4() {
        let input = "
hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        assert!(!validate_passport_part1(input));
    }

    #[test]
    fn field_validation() {
        assert!(validate_byr("2002"));
        assert!(!validate_byr("2003"));
        assert!(validate_hgt("60in"));
        assert!(validate_hgt("190cm"));
        assert!(!validate_hgt("190in"));
        assert!(!validate_hgt("190"));
        assert!(validate_hcl("#123abc"));
        assert!(!validate_hcl("#123abz"));
        assert!(!validate_hcl("123abc"));
        assert!(validate_ecl("brn"));
        assert!(!validate_ecl("wat"));
        assert!(validate_pid("000000001"));
        assert!(!validate_pid("0123456789"));
    }
}
