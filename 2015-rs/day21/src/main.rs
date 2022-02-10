#[derive(Debug)]
struct Character {
    hit: i32,
    damage: u32,
    armor: u32,
}

#[derive(Debug)]
struct Item {
    cost: u32,
    damage: u32,
    armor: u32,
}

#[derive(Debug)]
struct Shop {
    weapons: Vec<Item>,
    armor: Vec<Item>,
    rings: Vec<Item>,
}

impl Item {
    fn new(cost: u32, damage: u32, armor: u32) -> Item {
        Item {
            cost,
            damage,
            armor,
        }
    }
}

impl Shop {
    fn init() -> Shop {
        Shop {
            weapons: vec![
                Item::new(8, 4, 0),
                Item::new(10, 5, 0),
                Item::new(25, 6, 0),
                Item::new(40, 7, 0),
                Item::new(74, 9, 0),
            ],
            armor: vec![
                Item::new(0, 0, 0),
                Item::new(13, 0, 1),
                Item::new(31, 0, 2),
                Item::new(53, 0, 3),
                Item::new(75, 0, 4),
                Item::new(102, 0, 5),
            ],
            rings: vec![
                Item::new(0, 0, 0),
                Item::new(25, 1, 0),
                Item::new(50, 2, 0),
                Item::new(100, 3, 0),
                Item::new(20, 0, 1),
                Item::new(40, 0, 2),
                Item::new(80, 0, 3),
            ],
        }
    }
}

fn play(player: Character, boss: Character) -> bool {
    let mut player = player;
    let mut boss = boss;

    loop {
        boss.hit -= (player.damage.checked_sub(boss.armor).unwrap_or(1)) as i32;
        if boss.hit.le(&0) {
            break true;
        }
        // println!("The player deals {} - {} damage; the boss goes down to {} hit points.", player.damage, boss.armor, boss.hit);
        player.hit -= (boss.damage.checked_sub(player.armor).unwrap_or(1)) as i32;
        if player.hit.le(&0) {
            break false;
        }
        // println!("The boss deals {} - {} damage; the player goes down to {} hit points.", boss.damage, player.armor, player.hit);
    }
}

#[allow(unused_variables)]
fn solve(player_hit: i32, lose: bool) -> u32 {
    let shop = Shop::init();
    let mut gold: u32 = if lose { u32::MIN } else { u32::MAX };

    for w in &shop.weapons {
        for a in &shop.armor {
            for rr in &shop.rings {
                for lr in &shop.rings {
                    if rr.cost == lr.cost {
                        continue;
                    }
                    let player = Character {
                        hit: player_hit,
                        damage: w.damage + a.damage + rr.damage + lr.damage,
                        armor: w.armor + a.armor + rr.armor + lr.armor,
                    };

                    let boss = Character {
                        hit: 104,
                        damage: 8,
                        armor: 1,
                    };

                    let g = w.cost + a.cost + rr.cost + lr.cost;
                    gold = match (play(player, boss), lose) {
                        (true, false) => gold.min(g),
                        (false, true) => gold.max(g),
                        _ => gold,
                    }
                }
            }
        }
    }

    gold
}

fn main() {
    let solution_1 = solve(100, false);
    println!("Solution 1 -> {solution_1}");

    let solution_2 = solve(100, true);
    println!("Solution 2 -> {solution_2}");
}
