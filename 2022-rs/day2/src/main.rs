enum RoundScore {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

enum Scores {
    None = 0,
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn resolve_hand(i: &str) -> Scores {
    match i {
        "A" | "X" => Scores::Rock,
        "B" | "Y" => Scores::Paper,
        "C" | "Z" => Scores::Scissors,
        _ => Scores::None,
    }
}

fn solve_first(input: &str) -> u32 {
    let mut score: u32 = 0;

    let result = |s: i32| match s {
        0 => RoundScore::Draw,
        -2 | 1 => RoundScore::Loss,
        2 | -1 => RoundScore::Win,
        _ => RoundScore::Loss,
    } as u32;

    for round in input.lines() {
        let (oponent, me) = round.split_once(" ").unwrap();

        let me = resolve_hand(me) as i32;
        let oponent = resolve_hand(oponent) as i32;

        let s = oponent - me;

        score += result(s);
        score += me as u32;
    }

    score
}

fn solve_second(input: &str) -> u32 {
    let scores = [3, 1, 2, 3, 1];

    let mut score: u32 = 0;

    let resolve_play = |b: &str| match b {
        "X" => (RoundScore::Loss, -1),
        "Y" => (RoundScore::Draw, 0),
        "Z" => (RoundScore::Win, 1),
        _ => (RoundScore::Loss, 0),
    };

    for round in input.lines() {
        let (oponent, me) = round.split_once(" ").unwrap();

        let oponent = resolve_hand(oponent);

        let (result, me) = resolve_play(me);

        let index = oponent as i32 + me;

        score += scores[index as usize] as u32;
        score += result as u32;
    }

    score
}

fn main() {
    let input = include_str!("../input.txt");
    let first = solve_first(input);

    println!("Problem 1 -> {first}");

    let second = solve_second(input);

    println!("Problem 2 -> {second}");
}

#[test]
fn test_first() {
    let input = "A Y\nB X\nC Z";

    assert_eq!(15, solve_first(input));
}

#[test]
fn test_second() {
    let input = "A Y\nB X\nC Z";

    assert_eq!(12, solve_second(input));
}
