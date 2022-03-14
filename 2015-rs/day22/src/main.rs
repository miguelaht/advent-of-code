#[allow(dead_code)]
struct Spell {
    cost: u32,
    damage: u32,
    heals: u32,
    turns: u32,
    armor: u32,
    mana: u32,
}

#[allow(dead_code)]
struct Player {
    mana: u32,
    hit_points: i32,
    armor: u32,
    damage: u32,
    spells: Vec<Spell>,
}

impl Spell {
    fn new(cost: u32, damage: u32, heals: u32, turns: u32, armor: u32, mana: u32) -> Spell {
        Spell {
            cost,
            damage,
            heals,
            turns,
            armor,
            mana,
        }
    }
}

fn play(mut player: Player, mut boss: Player) -> Option<u32> {
    boss.hit_points -= player.damage as i32;
    player.hit_points -= (boss.damage.checked_sub(player.armor).unwrap_or(1)) as i32;

    Some(0)
}

fn solve(hit_points: i32, mana: u32) -> u32 {
    let spells: Vec<Spell> = vec![
        Spell::new(53, 4, 0, 0, 0, 0),
        Spell::new(73, 2, 2, 0, 0, 0),
        Spell::new(113, 0, 0, 6, 7, 0),
        Spell::new(173, 3, 0, 6, 0, 0),
        Spell::new(229, 3, 0, 6, 0, 101),
    ];

    let mut min_mana = u32::MAX;
    for spell in spells {
        let player = Player {
            mana,
            hit_points,
            armor: 0,
            damage: 0,
            spells: vec![spell],
        };
        let boss = Player {
            mana: 0,
            hit_points: 55,
            armor: 0,
            damage: 8,
            spells: vec![],
        };

        let r = play(player, boss);

        min_mana = match r {
            Some(n) => min_mana.min(n),
            None => min_mana,
        };
    }

    min_mana
}

fn main() {
    let solution_1 = solve(50, 500);
    println!("Solution 1 -> {solution_1}");
}
