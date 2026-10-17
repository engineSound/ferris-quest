// ✅ SOLUTION: peek only when you're stuck, then go back and type it yourself!
//! 🦀 QUEST 14: 🐉 THE FINAL BOSS 🐉
//!
//! The Borrow Dragon guards the treasure of Rust Island!
//! Use EVERYTHING you've learned: structs, enums, match, loops, Vec, Option.
//!
//! ⚔️  THE RULES OF BATTLE (one turn = the hero acts, then the dragon acts):
//!   Hero actions:
//!     Attack       → dragon loses `hero.power` HP
//!     Heal         → hero gains 30 HP (never above `max_hp`)
//!     Shield       → the dragon's next attack does 0 damage
//!   Then, if the dragon is still alive (HP > 0), it bites: hero loses 20 HP
//!   (or 0 if the hero used Shield this turn).
//!
//!   - Dragon HP hits 0 or below → Victory { turns } (the dragon doesn't bite back!)
//!   - Hero HP hits 0 or below   → Defeat { turns }
//!   - Out of actions            → StillFighting
//!
//! There's no step-by-step hint here. You've got this, Rustacean! 💪
//! (If you're REALLY stuck, `solutions/lesson_14_final_boss.rs` is waiting.)

#![allow(dead_code, unused_variables)]

#[derive(Debug, Clone, Copy, PartialEq)]
enum Action {
    Attack,
    Heal,
    Shield,
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Victory { turns: u32 },
    Defeat { turns: u32 },
    StillFighting,
}

struct Hero {
    hp: i32,
    max_hp: i32,
    power: i32,
}

struct Dragon {
    hp: i32,
}

const DRAGON_BITE: i32 = 20;
const HEAL_AMOUNT: i32 = 30;

fn battle(hero: &mut Hero, dragon: &mut Dragon, actions: &[Action]) -> Outcome {
    let mut turns = 0;
    for action in actions {
        turns += 1;
        let mut shielded = false;
        match action {
            Action::Attack => dragon.hp -= hero.power,
            Action::Heal => hero.hp = (hero.hp + HEAL_AMOUNT).min(hero.max_hp),
            Action::Shield => shielded = true,
        }
        if dragon.hp <= 0 {
            return Outcome::Victory { turns };
        }
        if !shielded {
            hero.hp -= DRAGON_BITE;
        }
        if hero.hp <= 0 {
            return Outcome::Defeat { turns };
        }
    }
    Outcome::StillFighting
}

/// Bonus: suggest the next move! Shield if the hero would die from one bite,
/// Heal if the hero is below half HP, otherwise Attack.
fn best_move(hero: &Hero) -> Action {
    if hero.hp <= DRAGON_BITE {
        Action::Shield
    } else if hero.hp < hero.max_hp / 2 {
        Action::Heal
    } else {
        Action::Attack
    }
}

// ───────────────────────── 🧪 Tests ─────────────────────────

fn new_hero() -> Hero {
    Hero { hp: 100, max_hp: 100, power: 25 }
}

#[test]
fn a_quick_victory() {
    let mut hero = new_hero();
    let mut dragon = Dragon { hp: 50 };
    let outcome = battle(&mut hero, &mut dragon, &[Action::Attack, Action::Attack]);
    assert_eq!(outcome, Outcome::Victory { turns: 2 });
    assert_eq!(hero.hp, 80); // bitten once, the 2nd attack finished the dragon
}

#[test]
fn shields_up() {
    let mut hero = new_hero();
    let mut dragon = Dragon { hp: 500 };
    let outcome = battle(&mut hero, &mut dragon, &[Action::Shield, Action::Shield]);
    assert_eq!(outcome, Outcome::StillFighting);
    assert_eq!(hero.hp, 100);
}

#[test]
fn healing_has_a_limit() {
    let mut hero = new_hero();
    let mut dragon = Dragon { hp: 500 };
    battle(&mut hero, &mut dragon, &[Action::Heal]);
    assert_eq!(hero.hp, 80); // healed to 100 (not 130!), then bitten
}

#[test]
fn a_tragic_defeat() {
    let mut hero = Hero { hp: 30, max_hp: 100, power: 1 };
    let mut dragon = Dragon { hp: 500 };
    let outcome = battle(&mut hero, &mut dragon, &[Action::Attack, Action::Attack, Action::Attack]);
    assert_eq!(outcome, Outcome::Defeat { turns: 2 });
}

#[test]
fn the_wise_hero() {
    assert_eq!(best_move(&Hero { hp: 15, max_hp: 100, power: 10 }), Action::Shield);
    assert_eq!(best_move(&Hero { hp: 40, max_hp: 100, power: 10 }), Action::Heal);
    assert_eq!(best_move(&Hero { hp: 90, max_hp: 100, power: 10 }), Action::Attack);
}

#[test]
fn defeat_the_borrow_dragon() {
    let mut hero = new_hero();
    let mut dragon = Dragon { hp: 200 };
    let mut actions = Vec::new();
    // Let the wise hero pick each move, one turn at a time.
    let mut outcome = Outcome::StillFighting;
    while outcome == Outcome::StillFighting && actions.len() < 100 {
        let next = best_move(&hero);
        actions.push(next);
        outcome = battle(&mut hero, &mut dragon, &[next]);
    }
    assert!(matches!(outcome, Outcome::Victory { .. }));
    println!("🏆 Victory in {} turns! The treasure of Rust Island is yours! 🦀👑", actions.len());
}
