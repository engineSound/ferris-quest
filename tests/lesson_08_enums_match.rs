//! 🦀 QUEST 8: The Weather Station ☀️🌧️❄️
//!
//! Ferris needs to know what to wear today!
//!
//! 📚 YOU'LL LEARN: `enum`, variants that carry data, `match`, `if let`.
//!
//! 🧠 `match` must handle EVERY case. If you forget one, the compiler tells you.
//!    No more forgotten cases! 🎉

#![allow(dead_code, unused_variables)]

#[derive(Debug, PartialEq)]
enum Weather {
    Sunny,
    Rainy,
    Snowy { cm: u32 },  // how deep the snow is
    Windy(u32),         // wind speed in km/h
}

/// What should Ferris wear?
///   Sunny            → "sunglasses 😎"
///   Rainy            → "umbrella ☂️"
///   Snowy, < 10 cm   → "boots 🥾"
///   Snowy, >= 10 cm  → "snowsuit ⛄"
///   Windy            → "kite 🪁"
///
/// 💡 HINT:
///   match weather {
///       Weather::Sunny => ...,
///       Weather::Snowy { cm } if *cm < 10 => ...,   // a "match guard"
///       ...
///   }
fn outfit(weather: &Weather) -> &'static str {
    todo!()
}

/// Ferris may go to the beach when it's sunny, or windy below 30 km/h.
///
/// 💡 HINT: `matches!(weather, Weather::Sunny)` returns a bool,
///    or just use a `match` that returns true/false.
fn beach_day(weather: &Weather) -> bool {
    todo!()
}

/// Turn a word into weather. Unknown words → `None`.
/// "sunny" → Some(Sunny), "rainy" → Some(Rainy), anything else → None
///
/// 💡 HINT: you can `match` on strings too! Use `_ =>` for "everything else".
fn parse_weather(word: &str) -> Option<Weather> {
    todo!()
}

// ───────────────────────── 🧪 Tests ─────────────────────────

#[test]
fn what_to_wear() {
    assert_eq!(outfit(&Weather::Sunny), "sunglasses 😎");
    assert_eq!(outfit(&Weather::Rainy), "umbrella ☂️");
    assert_eq!(outfit(&Weather::Snowy { cm: 3 }), "boots 🥾");
    assert_eq!(outfit(&Weather::Snowy { cm: 40 }), "snowsuit ⛄");
    assert_eq!(outfit(&Weather::Windy(50)), "kite 🪁");
}

#[test]
fn to_the_beach() {
    assert!(beach_day(&Weather::Sunny));
    assert!(beach_day(&Weather::Windy(10)));
    assert!(!beach_day(&Weather::Windy(80)));
    assert!(!beach_day(&Weather::Rainy));
}

#[test]
fn reading_the_forecast() {
    assert_eq!(parse_weather("sunny"), Some(Weather::Sunny));
    assert_eq!(parse_weather("rainy"), Some(Weather::Rainy));
    assert_eq!(parse_weather("meatballs"), None);
}
