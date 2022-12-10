#[derive(Debug, PartialEq)]
struct Instruction {
    qty: u32,
    from: u32,
    to: u32,
}

fn parse_input(input: &str, n: usize) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let mut positions: [usize; 9] = [1, 5, 9, 13, 17, 21, 25, 29, 33];
    (&mut positions[..n]).reverse();

    let (crates, instructions) = input.split_once("\n\n").unwrap();

    let mut stacks: Vec<Vec<char>> = std::iter::repeat(vec![]).take(n).collect::<Vec<_>>();

    crates.lines().for_each(|line| {
        let mut stack: usize = 0;
        for p in positions {
            let mut chrs = line.chars();
            let c = &chrs.nth(p);

            match c {
                Some(c) => {
                    if c.is_alphabetic() {
                        let s = stacks.get_mut(stack).unwrap();
                        s.insert(0, *c);
                    }
                }
                None => {}
            };

            stack += 1;
        }
    });
    let instructions = instructions
        .lines()
        .map(|instr| {
            let parts = instr.split(" ").collect::<Vec<&str>>();

            let qty = parts.get(1).unwrap().parse::<u32>().unwrap();
            let from = parts.get(3).unwrap().parse::<u32>().unwrap();
            let to = parts.get(5).unwrap().parse::<u32>().unwrap();

            Instruction { qty, from, to }
        })
        .collect::<Vec<Instruction>>();

    stacks.reverse();
    (stacks, instructions)
}

fn solve(input: &str, n: usize, reverse: bool) -> String {
    let (mut crates, instructions) = parse_input(input, n);

    for i in instructions {
        let from = crates.get_mut((i.from - 1) as usize).unwrap();
        let final_len = &from.len() - i.qty as usize;

        let p = &mut from.split_off(final_len);
        if reverse {
            p.reverse();
        }

        let to = crates.get_mut((i.to - 1) as usize).unwrap();
        to.append(p);
    }
    crates.iter().map(|s| s.last().unwrap()).collect::<String>()
}

fn main() {
    let input = include_str!("../input.txt");

    let p1 = solve(input, 9, true);
    println!("Problem 1 {p1}");

    let p2 = solve(input, 9, false);
    println!("Problem 2 {p2}");
}

#[test]
fn test_parse() {
    let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    let p = parse_input(input, 3);
    assert_eq!(
        p.1,
        &[
            Instruction {
                qty: 1,
                from: 2,
                to: 1,
            },
            Instruction {
                qty: 3,
                from: 1,
                to: 3,
            },
            Instruction {
                qty: 2,
                from: 2,
                to: 1,
            },
            Instruction {
                qty: 1,
                from: 1,
                to: 2,
            },
        ],
    );

    assert_eq!(p.0, &[vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]);
}

#[test]
fn test_first() {
    let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    assert_eq!(solve(input, 3, true), "CMZ".to_string());
}

#[test]
fn test_second() {
    let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    assert_eq!(solve(input, 3, false), "MCD".to_string());
}
