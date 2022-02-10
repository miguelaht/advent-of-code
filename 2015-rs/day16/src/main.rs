use fancy_regex::Regex;
use std::collections::HashMap;

fn parse(inp: &str) -> HashMap<&str, u32> {
    Regex::new(r"\w+: \d")
        .unwrap()
        .find_iter(inp)
        .map(|m| {
            let x: Vec<&str> = m.unwrap().as_str().split(": ").collect();
            (x[0], x[1].parse::<u32>().unwrap())
        })
        .collect()
}

fn solve_1(inp: &str) -> u32 {
    let the_sue: HashMap<&str, u32> = HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);

    let mut id = 0;
    let s = inp
        .lines()
        .map(|s| {
            id += 1;
            let mut sue = parse(s);
            sue.insert("id", id);

            let mut count = 0;

            for key in the_sue.keys() {
                count += match sue.get_key_value(key) {
                    Some(v) => {
                        if v.1.eq(&the_sue[key]) {
                            1
                        } else {
                            0
                        }
                    }
                    _ => 0,
                };
            }

            (sue["id"], count)
        })
        .max_by(|x, y| x.1.cmp(&y.1));

    s.unwrap().0
}

fn solve_2(inp: &str) -> u32 {
    let the_sue: HashMap<&str, u32> = HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);
    let mut id = 0;
    let s = inp
        .lines()
        .map(|s| {
            id += 1;
            let mut sue = parse(s);
            sue.insert("id", id);

            let mut count = 0;

            for key in the_sue.keys() {
                count += match sue.get_key_value(key) {
                    Some(v) => match *v.0 {
                        "cats" | "trees" => {
                            if v.1.gt(the_sue.get(v.0).unwrap()) {
                                1
                            } else {
                                0
                            }
                        }
                        "pomeranians" | "goldfish" => {
                            if v.1.lt(the_sue.get(v.0).unwrap()) {
                                1
                            } else {
                                0
                            }
                        }
                        _ => {
                            if v.1.eq(&the_sue[key]) {
                                1
                            } else {
                                0
                            }
                        }
                    },
                    _ => 0,
                };
            }

            (sue["id"], count)
        })
        .max_by(|x, y| x.1.cmp(&y.1));

    s.unwrap().0
}
fn main() {
    let inp = include_str!("../input.txt");
    println!("Solution 1 -> {}", solve_1(inp));
    println!("Solution 2 -> {}", solve_2(inp));
}
