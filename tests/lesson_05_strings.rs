//! 🦀 QUEST 5: The Message in a Bottle 🍾
//!
//! Ferris found bottles with secret messages! Let's decode them.
//!
//! 📚 YOU'LL LEARN: `&str` vs `String`, `.chars()`, `.split_whitespace()`, `.trim()`.
//!
//! 🧠 THE BIG IDEA:
//!   `String` = text you OWN (you can grow and change it).
//!   `&str`   = a peek at text that lives somewhere else (a "string slice").

#![allow(dead_code, unused_variables)]

/// The first word of the message. `first_word("ahoy there matey") == "ahoy"`
///
/// 💡 HINT: `message.split_whitespace().next()` gives an `Option<&str>`.
///    `.unwrap_or("")` turns it into a plain `&str`.
fn first_word(message: &str) -> &str {
    todo!()
}

/// Count vowels (a, e, i, o, u, upper or lower case).
///
/// 💡 HINT: `for c in message.chars()` and `"aeiouAEIOU".contains(c)`
fn count_vowels(message: &str) -> usize {
    todo!()
}

/// Secret messages are written backwards! `reverse("olleh") == "hello"`
///
/// 💡 HINT: `.chars().rev().collect()`. `collect` can build a `String`!
fn reverse(message: &str) -> String {
    todo!()
}

/// Clean up a soggy message: remove spaces at the ends and make it lowercase.
/// `clean("  HeLLo  ") == "hello"`
fn clean(message: &str) -> String {
    todo!()
}

// ───────────────────────── 🧪 Tests ─────────────────────────

#[test]
fn the_first_word() {
    assert_eq!(first_word("ahoy there matey"), "ahoy");
    assert_eq!(first_word("crab"), "crab");
    assert_eq!(first_word(""), "");
}

#[test]
fn vowel_counter() {
    assert_eq!(count_vowels("Ferris"), 2);
    assert_eq!(count_vowels("RUST IS AWESOME"), 6);
    assert_eq!(count_vowels("rhythm"), 0);
}

#[test]
fn backwards_message() {
    assert_eq!(reverse("olleh"), "hello");
    assert_eq!(reverse("!tsur"), "rust!");
}

#[test]
fn soggy_message() {
    assert_eq!(clean("  HeLLo  "), "hello");
    assert_eq!(clean("TREASURE\n"), "treasure");
}
