//! 🦀 QUEST 10: Ferris's Backpack 🎒
//!
//! Time to pack for an adventure!
//!
//! 📚 YOU'LL LEARN: `Vec<T>` (a growable list) and `HashMap<K, V>` (a dictionary).

#![allow(dead_code, unused_variables)]

use std::collections::HashMap;

/// Pack the starter items: a Vec containing "map", "snack", "flashlight" (in that order).
///
/// 💡 HINT: `vec!["a", "b"]` makes a Vec quickly.
fn starter_pack() -> Vec<&'static str> {
    todo!()
}

/// Add an item to the END of the backpack, unless it's already in there.
///
/// 💡 HINT: `backpack.contains(&item)` and `backpack.push(item)`
fn pack(backpack: &mut Vec<String>, item: &str) {
    todo!()
}

/// Count how many of each snack Ferris has.
/// `count_snacks(&["🍪", "🍩", "🍪"])` → {"🍪": 2, "🍩": 1}
///
/// 💡 HINT: the magic line is
///    `*counts.entry(snack).or_insert(0) += 1;`
fn count_snacks<'a>(snacks: &[&'a str]) -> HashMap<&'a str, u32> {
    todo!()
}

/// Look up the price of an item in the shop. Unknown items cost 0.
///
/// 💡 HINT: `shop.get(item)` returns an `Option<&u32>`. `.copied().unwrap_or(0)`
fn price(shop: &HashMap<String, u32>, item: &str) -> u32 {
    todo!()
}

// ───────────────────────── 🧪 Tests ─────────────────────────

#[test]
fn starter_items() {
    let pack = starter_pack();
    assert_eq!(pack.len(), 3);
    assert_eq!(pack[0], "map");
    assert_eq!(pack, vec!["map", "snack", "flashlight"]);
}

#[test]
fn no_duplicates_in_the_backpack() {
    let mut backpack = Vec::new();
    pack(&mut backpack, "rope");
    pack(&mut backpack, "compass");
    pack(&mut backpack, "rope");
    assert_eq!(backpack, vec!["rope", "compass"]);
}

#[test]
fn snack_inventory() {
    let counts = count_snacks(&["🍪", "🍩", "🍪", "🍪"]);
    assert_eq!(counts["🍪"], 3);
    assert_eq!(counts["🍩"], 1);
    assert_eq!(counts.get("🥦"), None); // no broccoli, phew
}

#[test]
fn shopping() {
    let mut shop = HashMap::new();
    shop.insert("tent".to_string(), 30);
    shop.insert("lantern".to_string(), 12);
    assert_eq!(price(&shop, "tent"), 30);
    assert_eq!(price(&shop, "dragon"), 0);
}
