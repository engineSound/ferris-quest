//! 🦀 QUEST 11: The Animal Choir 🎤
//!
//! A trait is a promise: "anything with this trait can do these things."
//! Like a job description! 📋
//!
//! ⚠️  This lesson won't compile until you write `impl Singer for Crab`. 🔧
//!
//! 📚 YOU'LL LEARN: `trait`, `impl Trait for Type`, default methods, generics, `dyn`.

#![allow(dead_code, unused_variables)]

trait Singer {
    fn name(&self) -> String;
    fn sound(&self) -> String;

    /// A DEFAULT method: every Singer gets this for free!
    /// It should return e.g. "Ferris sings: click click!"
    fn sing(&self) -> String {
        todo!()
    }
}

struct Cat;
struct Crab {
    name: String,
}

impl Singer for Cat {
    fn name(&self) -> String {
        "Cat".to_string()
    }
    fn sound(&self) -> String {
        "meow".to_string()
    }
}

// 🎯 Implement `Singer` for `Crab`. A crab's name is its `name` field,
//    and its sound is "click click".
// impl Singer for Crab { ... }

/// Generics! This works for ANY type `T` that is a Singer.
/// Return the song sung twice, separated by " ", e.g. "Cat sings: meow! Cat sings: meow!"
fn encore<T: Singer>(singer: &T) -> String {
    todo!()
}

/// A choir of DIFFERENT animals. `Box<dyn Singer>` means "some Singer, any kind".
/// Return every animal's song in order.
fn choir(singers: &[Box<dyn Singer>]) -> Vec<String> {
    todo!()
}

// ───────────────────────── 🧪 Tests ─────────────────────────

#[test]
fn the_cat_sings() {
    assert_eq!(Cat.sing(), "Cat sings: meow!");
}

#[test]
fn the_crab_sings() {
    let ferris = Crab { name: "Ferris".to_string() };
    assert_eq!(ferris.sing(), "Ferris sings: click click!");
}

#[test]
fn one_more_time() {
    assert_eq!(encore(&Cat), "Cat sings: meow! Cat sings: meow!");
    let ferris = Crab { name: "Ferris".to_string() };
    assert_eq!(encore(&ferris), "Ferris sings: click click! Ferris sings: click click!");
}

#[test]
fn the_whole_choir() {
    let singers: Vec<Box<dyn Singer>> = vec![
        Box::new(Cat),
        Box::new(Crab { name: "Ferris".to_string() }),
    ];
    assert_eq!(
        choir(&singers),
        vec!["Cat sings: meow!", "Ferris sings: click click!"]
    );
}
