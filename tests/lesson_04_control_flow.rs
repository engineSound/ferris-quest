//! 🦀 QUEST 4: The Crab Dance Party 💃
//!
//! Ferris is DJ tonight. Help choreograph the dance!
//!
//! 📚 YOU'LL LEARN: `if`/`else`, `for` loops, ranges (`1..=5`), `while`, `loop`.

#![allow(dead_code, unused_variables)]

/// The dance move for beat number `beat`:
///   - divisible by 3 AND 5 → "PinchScuttle"
///   - divisible by 3       → "Pinch"
///   - divisible by 5       → "Scuttle"
///   - otherwise            → the number itself, e.g. "7"
///
/// 💡 HINT: `if` is an expression, so it can return a value!
///    `n.to_string()` turns a number into a String.
fn dance_move(beat: u32) -> String {
    todo!()
}

/// Countdown to the party! `countdown(3)` → `"3 2 1 🎉"`
///
/// 💡 HINT: `for n in (1..=from).rev() { ... }` counts backwards.
///    Build a `let mut s = String::new();` and use `s.push_str(...)`.
fn countdown(from: u32) -> String {
    todo!()
}

/// Add up all numbers from 1 to `n`. `sum_to(4) == 1 + 2 + 3 + 4 == 10`
fn sum_to(n: u32) -> u32 {
    todo!()
}

/// Ferris doubles the volume each song, starting at 1, until it is at least `target`.
/// How many songs does that take? `songs_until_loud(8) == 3` (1 → 2 → 4 → 8)
///
/// 💡 HINT: try a `while` loop.
fn songs_until_loud(target: u32) -> u32 {
    todo!()
}

// ───────────────────────── 🧪 Tests ─────────────────────────

#[test]
fn the_dance_moves() {
    assert_eq!(dance_move(1), "1");
    assert_eq!(dance_move(3), "Pinch");
    assert_eq!(dance_move(10), "Scuttle");
    assert_eq!(dance_move(30), "PinchScuttle");
}

#[test]
fn party_countdown() {
    assert_eq!(countdown(3), "3 2 1 🎉");
    assert_eq!(countdown(1), "1 🎉");
}

#[test]
fn adding_it_up() {
    assert_eq!(sum_to(4), 10);
    assert_eq!(sum_to(100), 5050);
}

#[test]
fn turn_it_up() {
    assert_eq!(songs_until_loud(1), 0);
    assert_eq!(songs_until_loud(8), 3);
    assert_eq!(songs_until_loud(9), 4);
}
