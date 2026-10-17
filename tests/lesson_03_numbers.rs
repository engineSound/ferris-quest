//! 🦀 QUEST 3: The Number Bakery 🧁
//!
//! Ferris got a job at the bakery! Help with the math.
//!
//! 📚 YOU'LL LEARN: integers (`u32`, `i32`), floats (`f64`), `%`, `as` casting.

#![allow(dead_code, unused_variables)]

/// Each box holds 6 cupcakes. How many boxes for `cupcakes`? Round UP!
/// `boxes_needed(12) == 2`, `boxes_needed(13) == 3`.
///
/// 💡 HINT: integer division `13 / 6` gives `2` (it chops off the rest).
///    `%` gives the remainder: `13 % 6 == 1`.
fn boxes_needed(cupcakes: u32) -> u32 {
    todo!()
}

/// Is this number even? 🤔
fn is_even(n: i32) -> bool {
    todo!()
}

/// The oven is in Celsius but the recipe says Fahrenheit.
/// Formula: F = C × 9 / 5 + 32
fn to_fahrenheit(celsius: f64) -> f64 {
    todo!()
}

/// Average rating of three customers (they give whole stars, 1 to 5).
///
/// 💡 HINT: you can't mix `u32` and `f64` in Rust! Convert with `stars as f64`.
fn average_stars(a: u32, b: u32, c: u32) -> f64 {
    todo!()
}

// ───────────────────────── 🧪 Tests ─────────────────────────

#[test]
fn cupcakes_fit_in_boxes() {
    assert_eq!(boxes_needed(12), 2);
    assert_eq!(boxes_needed(13), 3);
    assert_eq!(boxes_needed(0), 0);
}

#[test]
fn even_or_odd() {
    assert!(is_even(4));
    assert!(!is_even(7));
    assert!(is_even(-2));
}

#[test]
fn oven_temperature() {
    assert_eq!(to_fahrenheit(0.0), 32.0);
    assert_eq!(to_fahrenheit(100.0), 212.0);
}

#[test]
fn happy_customers() {
    assert_eq!(average_stars(5, 5, 5), 5.0);
    assert_eq!(average_stars(4, 5, 3), 4.0);
    assert!((average_stars(5, 4, 4) - 4.333).abs() < 0.01);
}
