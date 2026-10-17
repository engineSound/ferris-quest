//! 🦀 QUEST 7: The Pet Shop 🐶🐱🐹
//!
//! Ferris opened a pet shop! Every pet needs a name, a kind, and happiness.
//!
//! 📚 YOU'LL LEARN: `struct`, `impl`, methods, `self`, `&self`, `&mut self`.

#![allow(dead_code, unused_variables)]

#[derive(Debug, PartialEq)]
struct Pet {
    name: String,
    kind: String,
    happiness: u8, // 0 = sad 😢, 100 = ecstatic 🤩
}

impl Pet {
    /// Make a new pet. New pets start with happiness 50.
    ///
    /// 💡 HINT: Pet { name: name.to_string(), kind: ..., happiness: 50 }
    fn new(name: &str, kind: &str) -> Pet {
        todo!()
    }

    /// Playing makes a pet +20 happier, but never above 100!
    ///
    /// 💡 HINT: `self.happiness = (self.happiness + 20).min(100);`
    ///    (`&mut self` means this method is allowed to change the pet.)
    fn play(&mut self) {
        todo!()
    }

    /// A pet is happy when happiness is 70 or more.
    fn is_happy(&self) -> bool {
        todo!()
    }

    /// `"Biscuit the dog (50% happy)"`
    fn describe(&self) -> String {
        todo!()
    }
}

// ───────────────────────── 🧪 Tests ─────────────────────────

#[test]
fn adopt_a_pet() {
    let pet = Pet::new("Biscuit", "dog");
    assert_eq!(pet.name, "Biscuit");
    assert_eq!(pet.kind, "dog");
    assert_eq!(pet.happiness, 50);
}

#[test]
fn playtime() {
    let mut pet = Pet::new("Mochi", "hamster");
    assert!(!pet.is_happy());
    pet.play();
    assert!(pet.is_happy());
    assert_eq!(pet.happiness, 70);
}

#[test]
fn too_much_fun() {
    let mut pet = Pet::new("Luna", "cat");
    for _ in 0..10 {
        pet.play();
    }
    assert_eq!(pet.happiness, 100);
}

#[test]
fn name_tag() {
    let pet = Pet::new("Biscuit", "dog");
    assert_eq!(pet.describe(), "Biscuit the dog (50% happy)");
}
