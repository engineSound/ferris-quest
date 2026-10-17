//! 🦀 QUEST 12: The Potion Factory 🧪✨
//!
//! Iterators are conveyor belts 🏭: items roll along and you can transform,
//! filter, and collect them. Closures `|x| x + 1` are tiny functions you
//! write right where you need them.
//!
//! 📚 YOU'LL LEARN: `.iter()`, `.map()`, `.filter()`, `.sum()`, `.collect()`, closures.

#![allow(dead_code, unused_variables)]

/// Double the strength of every potion. [1, 2, 3] → [2, 4, 6]
///
/// 💡 HINT: `potions.iter().map(|p| p * 2).collect()`
fn double_strength(potions: &[u32]) -> Vec<u32> {
    todo!()
}

/// Keep only the strong potions (strength above 5).
///
/// 💡 HINT: `.filter(|p| **p > 5)`. Yes, two stars! Filter gives you a
///    reference to a reference. Or use `.copied()` first to get plain numbers.
fn strong_only(potions: &[u32]) -> Vec<u32> {
    todo!()
}

/// Total magic: sum of the squares of each potion. [1, 2, 3] → 1 + 4 + 9 = 14
fn total_magic(potions: &[u32]) -> u32 {
    todo!()
}

/// The longest potion name, or None if there are no potions.
///
/// 💡 HINT: `.max_by_key(|name| name.len())` and `.copied()`
fn longest_name<'a>(names: &[&'a str]) -> Option<&'a str> {
    todo!()
}

/// A function that MAKES functions! 🤯 Return a closure that adds `bonus`.
/// `let boost = make_booster(10); boost(5) == 15`
///
/// 💡 HINT: `move |x| x + bonus`  (`move` lets the closure keep `bonus`)
fn make_booster(bonus: u32) -> impl Fn(u32) -> u32 {
    |x| todo!()
}

// ───────────────────────── 🧪 Tests ─────────────────────────

#[test]
fn double_double() {
    assert_eq!(double_strength(&[1, 2, 3]), vec![2, 4, 6]);
    assert_eq!(double_strength(&[]), Vec::<u32>::new());
}

#[test]
fn only_the_strong() {
    assert_eq!(strong_only(&[3, 8, 5, 10]), vec![8, 10]);
}

#[test]
fn magic_sum() {
    assert_eq!(total_magic(&[1, 2, 3]), 14);
}

#[test]
fn fancy_names() {
    assert_eq!(longest_name(&["fizz", "glimmerdust", "zap"]), Some("glimmerdust"));
    assert_eq!(longest_name(&[]), None);
}

#[test]
fn power_up() {
    let boost = make_booster(10);
    assert_eq!(boost(5), 15);
    let tiny = make_booster(1);
    assert_eq!(tiny(1), 2);
}
