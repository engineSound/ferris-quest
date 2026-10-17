# 🦀 Ferris Quest

**Learn Rust the happy way.** 14 small quests, no boring theory dumps, and a
friendly crab cheering you on. 🎉

```
         _~^~^~_
     \) /  o o  \ (/
       '_   -   _'
       / '-----' \
```

Every quest is one file of **small puzzles**. You fill in the blanks, click ▶,
and the tests go green. Each one is a small win. 🟩

---

## 🚀 Getting started (5 minutes)

1. **Install Rust** (skip this if you already have it): <https://rustup.rs>
2. **Open this folder in RustRover**: *File → Open…* → pick the `ferris-quest` folder.
   RustRover finds `Cargo.toml` by itself. Wait for the indexing bar at the bottom to finish.
3. **Open your Quest Map**: pick **🗺️ Quest Map** in the run dropdown (top right) and press ▶.
   Or in the terminal: `cargo run`
4. Open `tests/lesson_01_hello_ferris.rs` and start playing!

## 🎮 How to play

| You see…                     | It means…                                   | Do this                                   |
|------------------------------|---------------------------------------------|-------------------------------------------|
| `todo!()`                    | A blank for you to fill in                  | Replace it with your code                 |
| 🔧 `FIX:` comment            | Code that's broken on purpose               | Read the compiler error, then fix it      |
| 💡 `HINT:`                   | A nudge in the right direction              | Read it when you want help                |
| Green ▶ next to `#[test]`    | A test you can run                          | Click it! Green = win 🎉                  |

**Running tests in RustRover:**
- Click the ▶ in the gutter next to `#[test]` to run **one** test
- Click the ▶ next to the first line of the file to run the **whole lesson**
- Hover any red squiggle to read what the compiler says. It's really helpful!

**Running tests in the terminal:**
```bash
cargo run                                    # 🗺️  see your progress
cargo test --test lesson_01_hello_ferris     # run one lesson
```

## 🗺️ The Quests

| #  | Quest                         | You'll learn                                  |
|----|-------------------------------|-----------------------------------------------|
| 1  | 👋 Hello, Ferris!             | functions, `String`, `format!`                |
| 2  | 🏘️ The Variable Village 🔧    | `let`, `mut`, shadowing, tuples               |
| 3  | 🧁 The Number Bakery          | integers, floats, `%`, `as`                   |
| 4  | 💃 The Crab Dance Party       | `if`, `for`, `while`, ranges                  |
| 5  | 🍾 Message in a Bottle        | `&str` vs `String`, `.chars()`                |
| 6  | 🌊 The Ownership Ocean 🔧     | ownership, borrowing `&` / `&mut`, `.clone()` |
| 7  | 🐶 The Pet Shop               | `struct`, `impl`, methods                     |
| 8  | ☀️ The Weather Station        | `enum`, `match`                               |
| 9  | 🏴‍☠️ The Treasure Hunt          | `Option`, `Result`                            |
| 10 | 🎒 Ferris's Backpack          | `Vec`, `HashMap`                              |
| 11 | 🎤 The Animal Choir 🔧        | traits, generics, `dyn`                       |
| 12 | 🧪 The Potion Factory         | iterators, closures                           |
| 13 | 📜 The Hero Registry          | the `?` operator, error handling              |
| 14 | 🐉 **THE FINAL BOSS**         | everything together!                          |

🔧 = this quest starts out **not compiling** on purpose. Fixing compiler errors
is a core Rust skill, and you'll get good at it fast.

## 🎁 Bonus

- 🎲 **Guess Ferris's Number**: `cargo run --bin guess` (or the **🎲 Guess Game** run config).
  After a few quests, open `src/bin/guess.rs`. You'll understand every line!

## 🆘 Stuck?

1. **Read the error message slowly.** Rust's errors are famously friendly and often say exactly what to do.
2. Re-read the 💡 HINT in the comment above the function.
3. Peek at `solutions/` for that lesson. No shame! Then **type it yourself** so it sticks.
4. Wanted to start over? `git checkout tests/lesson_XX_name.rs` resets one lesson.

## 💖 Tips for happy learning

- **Small sessions win.** One quest a day is perfect.
- **Red is not failure.** Every red test is the compiler teaching you something.
- **Celebrate!** Run `cargo run` after each quest and watch your ⭐ grow.
- **Commit your progress**: `git commit -am "Quest 3 done! 🧁"`

## 📚 Where to go next

- [The Rust Book](https://doc.rust-lang.org/book/): the official, free guide
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/): learn by reading small examples
- [Rustlings](https://github.com/rust-lang/rustlings): more exercises like these
- [Exercism Rust track](https://exercism.org/tracks/rust): puzzles with mentor feedback

Happy hacking, future Rustacean! 🦀✨
