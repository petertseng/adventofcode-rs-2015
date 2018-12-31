const HERO_HP: u16 = 100;

#[rustfmt::skip]
const WEAPONS: &[(u16, u16)] = &[
    (8, 4),
    (10, 5),
    (25, 6),
    (40, 7),
    (74, 8),
];

#[rustfmt::skip]
const ARMOURS: &[(u16, u16)] = &[
    (0, 0),
    (13, 1),
    (31, 2),
    (53, 3),
    (75, 4),
    (102, 5),
];

const RINGS: &[(u16, u16, u16)] = &[
    (0, 0, 0),
    (25, 1, 0),
    (50, 2, 0),
    (100, 3, 0),
    (20, 0, 1),
    (40, 0, 2),
    (80, 0, 3),
];

type Fighter = (u16, u16, u16);

fn hero_win(hero: Fighter, boss: Fighter) -> bool {
    let hero_turns = turns_to_win(hero.1, boss.0, boss.2);
    let boss_turns = turns_to_win(boss.1, hero.0, hero.2);
    hero_turns <= boss_turns
}

fn turns_to_win(attacker_damage: u16, defender_hp: u16, defender_armour: u16) -> u16 {
    let effective_damage = if defender_armour >= attacker_damage {
        1
    } else {
        attacker_damage - defender_armour
    };
    (f32::from(defender_hp) / f32::from(effective_damage)).ceil() as u16
}

fn main() {
    let nums = adventofcode::read_n_numbers(3);
    let boss = (nums[0], nums[1], nums[2]);

    let mut min_win = std::u16::MAX;
    let mut max_lose = 0;

    for weapon in WEAPONS {
        for armour in ARMOURS {
            for ring1 in RINGS {
                for ring2 in RINGS {
                    if ring1 == ring2 && ring1.0 != 0 {
                        continue;
                    }
                    let cost = weapon.0 + armour.0 + ring1.0 + ring2.0;
                    let hero = (
                        HERO_HP,
                        weapon.1 + ring1.1 + ring2.1,
                        armour.1 + ring1.2 + ring2.2,
                    );
                    if hero_win(hero, boss) {
                        if cost < min_win {
                            min_win = cost;
                        }
                    } else if cost > max_lose {
                        max_lose = cost;
                    }
                }
            }
        }
    }

    println!("{}", min_win);
    println!("{}", max_lose);
}

#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode::tests;

    tests! {
        test_turns {
            regular(10, 100, 0, 10);
            armour(11, 100, 1, 10);
            too_much_armour(11, 100, 12, 100);
        }
    }

    fn test_turns(dmg: u16, hp: u16, armour: u16, expect: u16) {
        assert_eq!(turns_to_win(dmg, hp, armour), expect);
    }
}
