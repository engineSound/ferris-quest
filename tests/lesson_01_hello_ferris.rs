//! 🦀 QUEST 1: Hello, Ferris!
//!
//! Ferris the crab just washed up on the beach of Rust Island and wants to
//! meet you. Let's write our very first Rust functions!
//!
//! 🎯 HOW TO PLAY
//!   1. Replace each `todo!()` with real code.
//!   2. Click the green ▶ next to a `#[test]` (or `cargo test --test lesson_01_hello_ferris`).
//!   3. Green = you win! Red = read the message, it tells you what went wrong.
//!
//! 📚 YOU'LL LEARN: functions, `&str`, `String`, `format!`, return values.

#![allow(dead_code, unused_variables)]

/// Say hello! `greet("Ada")` should return `"Hello, Ada! 🦀"`.
///
/// 💡 HINT: `format!("Hello, {name}!")` builds a brand-new `String`,
///    just like `println!` but it hands the text back to you instead of printing it.
fn greet(name: &str) -> String {
    todo!()
}

/// Ferris is excited! `shout("hi")` should return `"HI!!!"`.
///
/// 💡 HINT: strings have a `.to_uppercase()` method.
fn shout(word: &str) -> String {
    todo!()
}

/// Ferris found one more shell. Return `shells + 1`.
///
/// 💡 HINT: In Rust, the LAST expression in a function (with NO semicolon)
///    is the return value. No `return` keyword needed!
fn one_more_shell(shells: i32) -> i32 {
    todo!()
}

// ───────────────────────── 🧪 Tests (don't change these!) ─────────────────────────

#[test]
fn ferris_says_hello() {
    assert_eq!(greet("Ada"), "Hello, Ada! 🦀");
    assert_eq!(greet("you"), "Hello, you! 🦀");
}

#[test]
fn ferris_shouts_with_joy() {
    assert_eq!(shout("hi"), "HI!!!");
    assert_eq!(shout("rust is fun"), "RUST IS FUN!!!");
}

#[test]
fn ferris_collects_shells() {
    assert_eq!(one_more_shell(0), 1);
    assert_eq!(one_more_shell(41), 42);
}
