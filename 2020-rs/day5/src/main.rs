fn find_seat(pass: &str) -> u32 {
    let mut region = [0, 127];
    let mut seat = [0, 7];

    for c in pass.chars() {
        match c {
            'B' => region[0] += ((region[1] - region[0]) + 1) / 2,
            'F' => region[1] -= ((region[1] - region[0]) + 1) / 2,
            'R' => seat[0] += ((seat[1] - seat[0]) + 1) / 2,
            'L' => seat[1] -= ((seat[1] - seat[0]) + 1) / 2,
            _ => {}
        };
    }

    let row = if &pass[6..=6] == "F" {
        region[0]
    } else {
        region[1]
    };
    let col = if &pass[9..=9] == "L" {
        seat[0]
    } else {
        seat[1]
    };

    row * 8 + col
}

fn main() {
    let solution_1 = *include_str!("../input.txt")
        .split("\n")
        .map(|x| find_seat(x))
        .collect::<Vec<u32>>()
        .iter()
        .max()
        .unwrap();

    println!("Solution 1 -> {}", solution_1);

    let mut solution_2 = include_str!("../input.txt")
        .split("\n")
        .map(|x| find_seat(x))
        .collect::<Vec<u32>>();

    solution_2.sort();

    let mut index = 0;
    let seat = loop {
        let res = solution_2[index + 1] - solution_2[index];
        if res > 1 || res <= 0 {
            break solution_2[index] + 1;
        }

        index += 1;
    };

    println!("Solution 2 -> {}", seat);
}

#[test]
fn get_id_357() {
    assert_eq!(find_seat("FBFBBFFRLR"), 357);
}

#[test]
fn get_id_567() {
    assert_eq!(find_seat("BFFFBBFRRR"), 567);
}

#[test]
fn get_id_820() {
    assert_eq!(find_seat("BBFFBBFRLL"), 820);
}

#[test]
fn get_id_119() {
    assert_eq!(find_seat("FFFBBBFRRR"), 119);
}
