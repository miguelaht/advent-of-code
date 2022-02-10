fn problem_1(goal: usize) -> usize {
    let goal = goal / 10;
    let mut houses: Vec<usize> = vec![1; goal];
    let mut house = goal;

    for i in 2..goal {
        let mut j = i;

        while j < goal {
            houses[j] += i;

            if houses[j] >= goal && j < house {
                house = j;
            }

            j += i;
        }
    }

    house
}

fn problem_2(goal: usize) -> usize {
    let goal = goal / 10;
    let mut houses: Vec<usize> = vec![11; goal];
    let mut house = goal;

    for i in 2..goal {
        let mut j = i;
        let mut visits = 0;

        while j < goal {
            houses[j] += i * 11;

            if houses[j] >= goal * 10 && j < house {
                house = j;
            }

            j += i;
            visits += 1;
            if visits == 50 {
                break;
            }
        }
    }

    house
}

fn main() {
    let solution_1 = problem_1(34000000);
    println!("Solution 1 -> {solution_1}");

    let solution_2 = problem_2(34000000);
    println!("Solution 2 -> {solution_2}");
}
