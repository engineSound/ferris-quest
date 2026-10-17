// ✅ SOLUTION: peek only when you're stuck, then go back and type it yourself!
//! 🦀 QUEST 2: The Variable Village
//!
//! ⚠️  SURPRISE! This lesson starts out NOT COMPILING. 🔧
//! That's on purpose. Rust's compiler is famous for being a *helpful* teacher.
//! Look for the red squiggles in RustRover, hover them, and read the message.
//! It often tells you exactly how to fix it!
//!
//! 📚 YOU'LL LEARN: `let`, `mut`, shadowing, constants, type annotations.

#![allow(dead_code, unused_variables, unused_assignments)]

/// Constants are SHOUTY_CASE and always need a type.
const MAX_COINS: u32 = 100;

#[test]
fn variables_dont_change_unless_you_say_so() {
    // In Rust, variables are frozen ❄️ by default. To change one, add `mut`.
    // 🔧 FIX: make `coins` mutable.
    let mut coins = 0;
    coins += 5;
    coins += 5;
    assert_eq!(coins, 10);
}

#[test]
fn shadowing_makes_a_new_variable() {
    // You can re-use a name with a new `let`. This is called "shadowing".
    // The new variable can even have a different TYPE!
    let treasure = "42";
    // 🔧 FIX: create a NEW `treasure` that is the number 42.
    // 💡 HINT: let treasure: u32 = treasure.parse().unwrap();
    let treasure: u32 = treasure.parse().unwrap();
    assert_eq!(treasure + 1, 43);
}

#[test]
fn constants_are_forever() {
    // 🔧 FIX: replace the 0 so the test passes (use MAX_COINS!).
    let half = MAX_COINS / 2;
    assert_eq!(half, 50);
}

#[test]
fn tuples_hold_a_little_bit_of_everything() {
    let ferris = ("Ferris", 10, true); // (name, legs, loves_rust)
    // 🔧 FIX: "destructure" the tuple into three variables.
    // 💡 HINT: let (name, legs, loves_rust) = ferris;
    let (name, legs, loves_rust) = ferris;
    assert_eq!(name, "Ferris");
    assert_eq!(legs, 10);
    assert!(loves_rust);
}
