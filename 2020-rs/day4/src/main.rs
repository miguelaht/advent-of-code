use std::collections::HashMap;

const FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn validate_value(field: &str, value: &str) -> bool {
    let valid = match field {
        "byr" => value.parse::<usize>().unwrap().wrapping_sub(1920) <= 82,
        "iyr" => value.parse::<usize>().unwrap().wrapping_sub(2010) <= 10,
        "eyr" => value.parse::<usize>().unwrap().wrapping_sub(2020) <= 10,
        "hgt" => {
            if value.ends_with("cm") && value.len() == 5 {
                value[0..3].parse::<usize>().unwrap().wrapping_sub(150) <= 43
            } else if value.ends_with("in") && value.len() == 4 {
                value[0..2].parse::<usize>().unwrap().wrapping_sub(59) <= 27
            } else {
                false
            }
        }
        "hcl" => value.len() == 7,
        "ecl" => COLORS.iter().any(|v| v == &value),
        "pid" => value.len() == 9,
        "cid" => true,
        _ => false,
    };

    valid
}

fn main() {
    let solution_1 = include_str!("../input.txt")
        .split("\n\n")
        .map(|x| {
            x.split_ascii_whitespace()
                .map(|field| field.split_once(':').unwrap())
                .collect::<HashMap<_, _>>()
        })
        .filter(|passport| FIELDS.iter().all(|key| passport.contains_key(key)))
        .count();

    println!("Solution 1 -> {}", solution_1);

    let solution_2 = include_str!("../input.txt")
        .split("\n\n")
        .map(|x| {
            x.split_ascii_whitespace()
                .map(|field| field.split_once(':').unwrap())
                .collect::<HashMap<_, _>>()
        })
        .filter(|passport| FIELDS.iter().all(|key| passport.contains_key(key)))
        .filter(|pass| pass.iter().all(|(k, v)| validate_value(k, v)))
        .count();

    println!("Solution 2 -> {}", solution_2);
}
