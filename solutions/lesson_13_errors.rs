// ✅ SOLUTION: peek only when you're stuck, then go back and type it yourself!
//! 🦀 QUEST 13: The Hero Registry 📜
//!
//! Heroes sign up with a line like "Ferris:7" (name:level).
//! Lots can go wrong! Let's handle it gracefully.
//!
//! 📚 YOU'LL LEARN: the `?` operator, `.ok_or()`, chaining Results.
//!
//! 🧠 `?` means: "if this is an Err, stop and return that Err right now.
//!    Otherwise, unwrap the Ok value and keep going." It's like a seatbelt. 🚗

#![allow(dead_code, unused_variables)]

#[derive(Debug, PartialEq)]
struct Hero {
    name: String,
    level: u32,
}

/// Parse "Ferris:7" into Hero { name: "Ferris", level: 7 }.
/// Errors:
///   no ':'           → Err("missing ':'")
///   empty name       → Err("hero needs a name")
///   level not number → Err("bad level")
///
/// 💡 HINT:
///   let (name, level) = line.split_once(':').ok_or("missing ':'")?;
///   let level: u32 = level.parse().map_err(|_| "bad level")?;
fn parse_hero(line: &str) -> Result<Hero, String> {
    let (name, level) = line.split_once(':').ok_or("missing ':'")?;
    if name.is_empty() {
        return Err("hero needs a name".to_string());
    }
    let level: u32 = level.parse().map_err(|_| "bad level")?;
    Ok(Hero { name: name.to_string(), level })
}

/// Parse MANY heroes, one per line. If ANY line fails, return that error.
///
/// 💡 HINT: `?` works inside a `for` loop too! Or, the fancy way:
///    `text.lines().map(parse_hero).collect()`
///    (collect can turn many Results into one Result of a Vec 🪄)
fn parse_party(text: &str) -> Result<Vec<Hero>, String> {
    text.lines().map(parse_hero).collect()
}

/// The total level of the party, using `?` to bubble up errors.
fn party_power(text: &str) -> Result<u32, String> {
    let party = parse_party(text)?;
    Ok(party.iter().map(|h| h.level).sum())
}

// ───────────────────────── 🧪 Tests ─────────────────────────

#[test]
fn a_good_hero() {
    assert_eq!(
        parse_hero("Ferris:7"),
        Ok(Hero { name: "Ferris".to_string(), level: 7 })
    );
}

#[test]
fn broken_signups() {
    assert_eq!(parse_hero("Ferris"), Err("missing ':'".to_string()));
    assert_eq!(parse_hero(":3"), Err("hero needs a name".to_string()));
    assert_eq!(parse_hero("Ferris:lots"), Err("bad level".to_string()));
}

#[test]
fn the_whole_party() {
    let party = parse_party("Ferris:7\nCorro:3").unwrap();
    assert_eq!(party.len(), 2);
    assert_eq!(party[1].name, "Corro");
    assert!(parse_party("Ferris:7\noops").is_err());
}

#[test]
fn party_strength() {
    assert_eq!(party_power("Ferris:7\nCorro:3"), Ok(10));
    assert_eq!(party_power("Ferris:7\nCorro:x"), Err("bad level".to_string()));
}
