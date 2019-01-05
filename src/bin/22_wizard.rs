use std::collections::HashMap;

type Mana = u16;
const MAX_MANA: Mana = std::u16::MAX;

#[derive(Clone, Copy)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn cost(self) -> Mana {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }
}

// Note that this affects spell search order.
const SPELLS: &[Spell] = &[
    Spell::Poison,
    Spell::Shield,
    Spell::Recharge,
    Spell::MagicMissile,
    Spell::Drain,
];

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Game {
    my_hp: i16,
    my_mp: Mana,
    boss_hp: i16,
    boss_damage: u8,
    shield_time: u8,
    poison_time: u8,
    recharge_time: u8,
    hard: bool,
}

impl Game {
    fn hard(&self) -> Self {
        Self {
            my_hp: self.my_hp - 1,
            hard: true,
            ..*self
        }
    }

    fn cast_spell(&self, s: Spell) -> Result<Game, bool> {
        let g = self.apply_spell_effect(s)?;
        let g = g.tick_timers()?;
        let g = g.boss_attack()?;
        let g = g.hard_mode_penalty()?;
        g.tick_timers()
    }

    fn legal(&self, s: Spell) -> bool {
        if s.cost() > self.my_mp {
            return false;
        }
        match s {
            Spell::MagicMissile => true,
            Spell::Drain => true,
            Spell::Shield => self.shield_time == 0,
            Spell::Poison => self.poison_time == 0,
            Spell::Recharge => self.recharge_time == 0,
        }
    }

    fn apply_spell_effect(&self, s: Spell) -> Result<Game, bool> {
        let g = Game {
            my_mp: self.my_mp - s.cost(),
            ..*self
        };
        let g2 = match s {
            Spell::MagicMissile => Game {
                boss_hp: g.boss_hp - 4,
                ..g
            },
            Spell::Drain => Game {
                boss_hp: g.boss_hp - 2,
                my_hp: self.my_hp + 2,
                ..g
            },
            Spell::Shield => Game {
                shield_time: 6,
                ..g
            },
            Spell::Poison => Game {
                poison_time: 6,
                ..g
            },
            Spell::Recharge => Game {
                recharge_time: 5,
                ..g
            },
        };
        g2.winner()
    }

    fn tick_timers(&self) -> Result<Game, bool> {
        let g0 = *self;
        let g1 = if g0.shield_time > 0 {
            Game {
                shield_time: g0.shield_time - 1,
                ..g0
            }
        } else {
            g0
        };
        let g2 = if g1.recharge_time > 0 {
            Game {
                recharge_time: g1.recharge_time - 1,
                my_mp: g1.my_mp + 101,
                ..g1
            }
        } else {
            g1
        };
        let g3 = if g2.poison_time > 0 {
            Game {
                poison_time: g2.poison_time - 1,
                boss_hp: g2.boss_hp - 3,
                ..g2
            }
        } else {
            g2
        };
        g3.winner()
    }

    fn boss_attack(&self) -> Result<Game, bool> {
        let shield = if self.shield_time > 0 { 7 } else { 0 };
        let damage = if self.boss_damage <= shield {
            1
        } else {
            i16::from(self.boss_damage - shield)
        };
        Game {
            my_hp: self.my_hp - damage,
            ..*self
        }
        .winner()
    }

    fn hard_mode_penalty(&self) -> Result<Game, bool> {
        Game {
            my_hp: self.my_hp - if self.hard { 1 } else { 0 },
            ..*self
        }
        .winner()
    }

    fn winner(self) -> Result<Game, bool> {
        if self.my_hp <= 0 {
            Err(false)
        } else if self.boss_hp <= 0 {
            Err(true)
        } else {
            Ok(self)
        }
    }
}

fn search(game: Game) -> Mana {
    let mut best = MAX_MANA;
    let mut seen = HashMap::new();
    search_(game, 0, &mut best, &mut seen)
}

fn search_(game: Game, cost_so_far: Mana, best: &mut Mana, seen: &mut HashMap<Game, Mana>) -> Mana {
    if *best <= cost_so_far {
        return MAX_MANA;
    }

    if let Some(&prev_cost) = seen.get(&game) {
        if prev_cost <= cost_so_far {
            return MAX_MANA;
        }
    }
    seen.insert(game, cost_so_far);

    let spell_results = SPELLS.iter().map(|&spell| {
        if !game.legal(spell) {
            return MAX_MANA;
        }
        let cost = cost_so_far + spell.cost();
        match game.cast_spell(spell) {
            Err(false) => MAX_MANA,
            Err(true) => {
                if cost < *best {
                    *best = cost;
                }
                cost
            }
            Ok(g) => search_(g, cost, best, seen),
        }
    });
    spell_results.min().unwrap_or(MAX_MANA)
}

fn main() {
    let nums: Vec<u8> = adventofcode::read_n_numbers(2);

    let g = Game {
        my_hp: 50,
        my_mp: 500,
        boss_hp: i16::from(nums[0]),
        boss_damage: nums[1],
        hard: false,
        shield_time: 0,
        poison_time: 0,
        recharge_time: 0,
    };
    println!("{}", search(g));
    println!("{}", search(g.hard()));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: Game = Game {
        my_hp: 10,
        my_mp: 250,
        boss_hp: 0,
        boss_damage: 8,
        hard: false,
        shield_time: 0,
        poison_time: 0,
        recharge_time: 0,
    };

    fn cast_spells(game: Game, spells: &[Spell]) -> Result<Game, bool> {
        spells
            .iter()
            .fold(Ok(game), |r, &s| r.and_then(|g| g.cast_spell(s)))
    }

    #[test]
    fn win_13() {
        let g = Game {
            boss_hp: 13,
            ..EXAMPLE
        };
        assert_eq!(
            Err(true),
            cast_spells(g, &[Spell::Poison, Spell::MagicMissile])
        );
    }

    #[test]
    fn havent_won_yet_13() {
        let g = Game {
            boss_hp: 13,
            ..EXAMPLE
        };
        assert!(cast_spells(g, &[Spell::Poison]).is_ok());
    }

    #[test]
    fn lose_14() {
        let g = Game {
            boss_hp: 14,
            ..EXAMPLE
        };
        assert_eq!(
            Err(false),
            cast_spells(g, &[Spell::Poison, Spell::MagicMissile])
        );
    }

    #[test]
    fn havent_won_yet_14() {
        let g = Game {
            boss_hp: 14,
            ..EXAMPLE
        };
        assert!(cast_spells(
            g,
            &[Spell::Recharge, Spell::Shield, Spell::Drain, Spell::Poison]
        )
        .is_ok());
    }

    #[test]
    fn win_14() {
        let g = Game {
            boss_hp: 14,
            ..EXAMPLE
        };
        assert_eq!(
            Err(true),
            cast_spells(
                g,
                &[
                    Spell::Recharge,
                    Spell::Shield,
                    Spell::Drain,
                    Spell::Poison,
                    Spell::MagicMissile
                ]
            )
        );
    }
}
