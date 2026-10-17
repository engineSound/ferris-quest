//! 🦀 QUEST 9: The Treasure Hunt 🏴‍☠️
//!
//! Rust has no `null`! Instead it has two friendly boxes:
//!   `Option<T>`    = Some(value) 🎁 or None 📭      ("maybe there's something")
//!   `Result<T, E>` = Ok(value)   ✅ or Err(error) ❌ ("it worked, or here's why not")
//!
//! 📚 YOU'LL LEARN: `Option`, `Result`, `Some`, `None`, `Ok`, `Err`, `.unwrap_or()`.

#![allow(dead_code, unused_variables)]

/// Find the position of the treasure 'X' on the map.
/// `find_treasure("..X.") == Some(2)`, `find_treasure("....") == None`
///
/// 💡 HINT: `map.find('X')` already returns an Option<usize>! 🎉
fn find_treasure(map: &str) -> Option<usize> {
    todo!()
}

/// How many steps to the treasure? If there's no treasure, it's 0 steps.
///
/// 💡 HINT: `.unwrap_or(0)`
fn steps_to_treasure(map: &str) -> usize {
    todo!()
}

/// Share gold coins equally with friends.
///   no friends → Err("no friends to share with 😢")
///   otherwise  → Ok(coins each)
fn share_gold(coins: u32, friends: u32) -> Result<u32, String> {
    todo!()
}

/// Read a number from the treasure chest's lock. Bad input → Err("not a number: <text>")
///
/// 💡 HINT: `text.parse::<u32>()` gives a Result. Use `.map_err(|_| format!(...))`
///    to swap the error for a friendlier message.
fn read_lock(text: &str) -> Result<u32, String> {
    todo!()
}

// ───────────────────────── 🧪 Tests ─────────────────────────

#[test]
fn x_marks_the_spot() {
    assert_eq!(find_treasure("..X."), Some(2));
    assert_eq!(find_treasure("X"), Some(0));
    assert_eq!(find_treasure("~~~~"), None);
}

#[test]
fn counting_steps() {
    assert_eq!(steps_to_treasure("....X"), 4);
    assert_eq!(steps_to_treasure("...."), 0);
}

#[test]
fn sharing_is_caring() {
    assert_eq!(share_gold(10, 2), Ok(5));
    assert_eq!(share_gold(10, 0), Err("no friends to share with 😢".to_string()));
}

#[test]
fn opening_the_lock() {
    assert_eq!(read_lock("1234"), Ok(1234));
    assert_eq!(read_lock("abc"), Err("not a number: abc".to_string()));
}
