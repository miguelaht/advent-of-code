use fancy_regex::Regex;

#[derive(Debug)]
struct Reindeer {
    speed: u32,
    time: u32,
    rest: u32,
    resting: u32,
    distance: u32,
    points: u32,
}

fn lead(g: &Vec<Reindeer>, prop: &str) -> u32 {
    *g.iter()
        .map(|r: &Reindeer| -> u32 {
            return match prop {
                "distance" => r.distance,
                "points" => r.points,
                _ => 0,
            };
        })
        .collect::<Vec<u32>>()
        .iter()
        .max()
        .unwrap()
}

fn parse(l: &str) -> Vec<Reindeer> {
    let re = Regex::new(r"\d+").unwrap();
    l.lines()
        .map(|l| {
            let m = re
                .find_iter(l)
                .map(|m| m.unwrap().as_str().parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            Reindeer {
                speed: m[0],
                time: m[1],
                rest: m[2],
                resting: 0,
                distance: 0,
                points: 0,
            }
        })
        .collect::<Vec<Reindeer>>()
}

fn solution_1(inp: &str, seconds: u32, extra_point: bool) -> u32 {
    let mut g = parse(inp);

    for _ in 0..seconds {
        g.iter_mut().for_each(|mut r| {
            if r.resting < r.time {
                r.distance += r.speed;
                r.resting += 1;
                if r.resting == r.time {
                    r.resting = r.rest + r.time;
                }
            } else {
                r.resting -= 1;
                if r.resting == r.time {
                    r.resting = 0;
                }
            }
        });

        if extra_point {
            let l = lead(&g, "distance");
            g.iter_mut().for_each(|mut r| {
                if r.distance == l {
                    r.points += 1;
                }
            });
        }
    }

    return if extra_point {
        lead(&g, "points")
    } else {
        lead(&g, "distance")
    };
}

fn main() {
    let inp = include_str!("../input.txt");
    println!("Solution 1 -> {}", solution_1(inp, 2503, false));
    println!("Solution 2 -> {}", solution_1(inp, 2503, true));
}

#[test]
fn test_solution_1() {
    let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\nDancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
    assert_eq!(solution_1(input, 1000, false), 1120);
}

#[test]
fn test_solution_2() {
    let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\nDancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
    assert_eq!(solution_1(input, 1000, true), 689);
}
