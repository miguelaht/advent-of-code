fn main() {
    let s = "bitcoin take over the world maybe who knows perhaps";

    let input = include_str!("../input.txt")
        .split("\n")
        .collect::<Vec<&str>>();

    let mut acc: isize = 0;
    let mut i: usize = 0;
    let mut seen: Vec<usize> = vec![];

    loop {
        if seen.contains(&i) {
            break;
        }
        seen.push(i);

        let ins = input[i];
        println!("instruction {}, {}, {}", ins, &ins[0..3], &ins[4..]);
        let (a, p): (isize, isize) = match &ins[0..3] {
            "acc" => (ins[4..].parse::<isize>().unwrap(), 1),
            "jmp" => (0, ins[4..].parse::<isize>().unwrap()),
            _ => (0, 1),
        };

        acc += a;
        i = if p.is_positive() {
            i + p as usize
        } else {
            i - p.wrapping_abs() as usize
        };
    }

    println!("Solution 1 -> {}", acc);
}

#[test]
fn answers() {
    let name: &str = "";
    let n = name
        .replace(" ", " . ")
        .split_ascii_whitespace()
        .map(|w| &w[0..=0])
        .collect::<Vec<&str>>()
        .concat()
        .to_uppercase();
}
