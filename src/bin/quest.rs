//! 🗺️  The Quest Map
//!
//! Run me with `cargo run` (or the "🗺️ Quest Map" run configuration in RustRover).
//! I check every lesson in `tests/` and show you how far you've come.

use std::fs;
use std::process::Command;

const FERRIS: &str = r#"
         _~^~^~_
     \) /  o o  \ (/
       '_   -   _'
       / '-----' \
"#;

enum Status {
    Done,
    InProgress { passed: u32, total: u32 },
    DoesNotCompile,
}

fn main() {
    let mut lessons: Vec<String> = fs::read_dir("tests")
        .expect("Run me from the project folder (where Cargo.toml lives) 🦀")
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| name.starts_with("lesson_") && name.ends_with(".rs"))
        .map(|name| name.trim_end_matches(".rs").to_string())
        .collect();
    lessons.sort();

    println!("{FERRIS}");
    println!("  🦀 Ferris says: \"Let's see how your quest is going...\"\n");

    let mut stars = 0;
    let mut next_up: Option<&String> = None;

    for lesson in &lessons {
        let status = check(lesson);
        let pretty = lesson.trim_start_matches("lesson_").replace('_', " ");
        match status {
            Status::Done => {
                stars += 1;
                println!("  ⭐ {pretty:<28} complete!");
            }
            Status::InProgress { passed, total } => {
                println!("  🔨 {pretty:<28} {passed}/{total} tests passing");
                next_up.get_or_insert(lesson);
            }
            Status::DoesNotCompile => {
                println!("  🔧 {pretty:<28} doesn't compile yet (that's normal!)");
                next_up.get_or_insert(lesson);
            }
        }
    }

    let total = lessons.len();
    let bar: String = (0..total).map(|i| if i < stars { '🟩' } else { '⬜' }).collect();
    println!("\n  Progress: {bar}  {stars}/{total} ⭐\n");

    match next_up {
        Some(lesson) => {
            println!("  👉 Next quest: open tests/{lesson}.rs");
            println!("     Click the green ▶ next to a test to run it, or:");
            println!("     cargo test --test {lesson}\n");
            println!("  {}", cheer(stars));
        }
        None => {
            println!("  🎉🎉🎉 YOU DID IT! Every quest is complete! 🎉🎉🎉");
            println!("  You are now an official Rustacean. Ferris is so proud. 🦀💖");
            println!("  Treat yourself: cargo run --bin guess\n");
        }
    }
}

/// Runs one lesson's tests quietly and works out how it went.
fn check(lesson: &str) -> Status {
    let output = Command::new(env!("CARGO"))
        .args(["test", "--quiet", "--color", "never", "--test", lesson])
        .output()
        .expect("could not run cargo");
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    // Looks for a line like: "test result: FAILED. 2 passed; 1 failed; ..."
    let Some(line) = text.lines().find(|l| l.starts_with("test result:")) else {
        return Status::DoesNotCompile;
    };
    let passed = number_before(line, " passed");
    let failed = number_before(line, " failed");
    if failed == 0 && passed > 0 {
        Status::Done
    } else {
        Status::InProgress { passed, total: passed + failed }
    }
}

fn number_before(line: &str, word: &str) -> u32 {
    line.split(';')
        .find(|part| part.contains(word))
        .and_then(|part| part.split_whitespace().rev().nth(1))
        .and_then(|n| n.parse().ok())
        .unwrap_or(0)
}

fn cheer(stars: usize) -> &'static str {
    match stars {
        0 => "🌱 Every Rustacean started right here. You've got this!",
        1..=3 => "🚀 Great start! The compiler is your friend, read what it says.",
        4..=7 => "🔥 You're on fire! Halfway to crab-hood.",
        8..=11 => "🏔️  Look how far you've climbed! The summit is close.",
        _ => "👑 The final boss awaits. Ferris believes in you!",
    }
}
