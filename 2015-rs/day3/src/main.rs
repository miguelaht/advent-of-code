use std::collections::HashMap;

fn solution_1(input: &str) -> usize {
    let mut houses: HashMap<(i16, i16), usize> = HashMap::new();
    houses.insert((0, 0), 1);
    let mut x = 0;
    let mut y = 0;

    input.chars().for_each(|c| {
        match c {
            'v' => y -= 1,
            '^' => y += 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => (),
        };

        let e = houses.entry((x, y)).or_default();
        *e += 1;
    });

    houses.keys().count()
}

fn solution_2(input: &str) -> usize {
    let mut houses: HashMap<(i16, i16), usize> = HashMap::new();
    houses.insert((0, 0), 2);
    let mut sx = 0;
    let mut sy = 0;
    let mut rx = 0;
    let mut ry = 0;
    let mut turn = true;

    input.chars().for_each(|c| {
        let (x, y) = match c {
            'v' => (0, -1),
            '^' => (0, 1),
            '>' => (1, 0),
            '<' => (-1, 0),
            _ => (0, 0),
        };

        if turn {
            //santa
            sx += x;
            sy += y;
            let e = houses.entry((sx, sy)).or_default();
            *e += 1;
        } else {
            // robo santa
            rx += x;
            ry += y;
            let e = houses.entry((rx, ry)).or_default();
            *e += 1;
        }
        turn = !turn;
    });

    houses.keys().count()
}

fn main() {
    println!("Solution 1 -> {}", solution_1(include_str!("../input.txt")));
    println!("Solution 2 -> {}", solution_2(include_str!("../input.txt")));
}

#[test]
fn test_solution_1() {
    assert_eq!(2, solution_1(">"));
}

#[test]
fn test_solution_2() {
    assert_eq!(3, solution_2("^v"));
}
