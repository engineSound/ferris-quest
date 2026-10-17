// ✅ SOLUTION: peek only when you're stuck, then go back and type it yourself!
//! 🦀 QUEST 6: The Ownership Ocean 🌊  (Rust's superpower!)
//!
//! ⚠️  This lesson starts out NOT COMPILING. 🔧 That's the whole point!
//!
//! 🧠 THE THREE RULES OF OWNERSHIP
//!   1. Every value has exactly ONE owner.
//!   2. When the owner goes away, the value is cleaned up. 🧹
//!   3. You can LEND a value out with `&` (look but don't touch)
//!      or `&mut` (you may change it), instead of giving it away.
//!
//! Think of it like a toy 🧸: you can GIVE it away (move), LET A FRIEND LOOK (`&`),
//! LET A FRIEND PLAY WITH IT (`&mut`), or buy a COPY (`.clone()`).
//!
//! 💡 Hover the red squiggles! The compiler explains "value borrowed here after move".

#![allow(dead_code, unused_variables)]

// 🔧 FIX 1: this function TAKES OWNERSHIP of the string, so the caller loses it.
//    Change it to BORROW instead: `fn letter_count(word: &str) -> usize`
fn letter_count(word: &str) -> usize {
    word.chars().count()
}

// 🔧 FIX 2: this function should CHANGE the caller's string, so it needs `&mut String`.
fn add_sparkle(word: &mut String) {
    word.push_str("✨");
}

#[test]
fn lend_it_dont_give_it() {
    let pet = String::from("Ferris");
    let n = letter_count(&pet);
    assert_eq!(n, 6);
    assert_eq!(pet, "Ferris"); // we still have our pet! 🦀
}

#[test]
fn let_a_friend_play_with_it() {
    let mut shell = String::from("shell");
    add_sparkle(&mut shell);
    assert_eq!(shell, "shell✨");
}

#[test]
fn make_a_copy() {
    let original = String::from("treasure map");
    // 🔧 FIX 3: `let copy = original;` MOVES the map. Make a real copy instead.
    let copy = original.clone();
    assert_eq!(original, copy);
}

#[test]
fn numbers_are_copied_for_free() {
    // Small simple things like numbers are `Copy`. They never move!
    // Nothing to fix here, just enjoy it. 😌
    let a = 5;
    let b = a;
    assert_eq!(a + b, 10);
}
