use fancy_regex::Regex;

struct Ingredient {
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

fn parse(inp: &str) -> Vec<Ingredient> {
    let re = Regex::new(r"(-?\d+)").unwrap();

    return inp
        .lines()
        .map(|i| {
            let n = re
                .find_iter(i)
                .map(|m| m.unwrap().as_str().parse::<isize>().unwrap())
                .collect::<Vec<isize>>();

            Ingredient {
                capacity: n[0],
                durability: n[1],
                flavor: n[2],
                texture: n[3],
                calories: n[4],
            }
        })
        .collect::<Vec<Ingredient>>();
}

fn solve(inp: &str, count_calories: bool) -> isize {
    let ingredients = parse(inp);
    let mut score = 0;
    for i in 0..100 {
        for j in 0..100 - i {
            for k in 0..100 - i - j {
                let l = 100 - i - j - k;
                let mut q = vec![&i, &j, &k, &l].into_iter();
                let mut specs: Vec<isize> = vec![0, 0, 0, 0, 0];

                ingredients.iter().for_each(|i| {
                    let n = *q.next().unwrap();
                    specs[0] += i.capacity * n;
                    specs[1] += i.durability * n;
                    specs[2] += i.flavor * n;
                    specs[3] += i.texture * n;
                    specs[4] += i.calories * n;
                });

                if count_calories && specs[4].ne(&500) {
                    continue;
                }

                if specs[0].is_positive()
                    && specs[1].is_positive()
                    && specs[2].is_positive()
                    && specs[3].is_positive()
                    && !specs.contains(&0)
                {
                    specs.pop();
                    let t: isize = specs.iter().product();
                    score = score.max(t);
                }
            }
        }
    }
    score
}

fn main() {
    let inp = include_str!("../input.txt");

    let solution_1 = solve(inp, false);
    println!("Solution 1 -> {solution_1}");

    let solution_2 = solve(inp, true);
    println!("Solution 2 -> {solution_2}");
}
