// To be implemented by structs that own
// the Cipher trait
trait Cipher {
    // Takes in a borrowed (takes no ownership of) message string
    // and returns encrypted message (and the receiver takes ownership)
    fn encrypt(&self, msg: &str) -> Option<String>;

    // Takes in a borrowed (takes no ownership of) message string
    // and returns decrypted message (and the receiver takes ownership)
    fn decrypt(&self, enc: &str) -> Option<String>;
}

struct Caesar {
    shift: u32,
}
impl Caesar {
    #![allow(dead_code)]
    fn new(shift: u32) -> Self {
        Self { shift }
    }

    fn set_shift(&mut self, _shift: u32) {
        self.shift = _shift;
    }
}
impl Default for Caesar {
    fn default() -> Self {
        Self { shift: 10 }
    }
}
impl Cipher for Caesar {
    fn encrypt(&self, msg: &str) -> Option<String> {
        let uppercase_msg = msg.to_uppercase();
        let mut chars: Vec<u32> = uppercase_msg.chars().map(|c| c as u32).collect();
        for char in &mut chars {
            if *char != (' ' as u32) {
                *char = (*char - 65 + self.shift) % 26 + 65;
            }
        }

        Some(
            chars
                .iter()
                .flat_map(|c| std::char::from_u32(*c))
                .collect::<String>()
                .to_owned(),
        )
    }

    fn decrypt(&self, enc: &str) -> Option<String> {
        // TODO: Fill in logic
        Some(enc.to_owned())
    }
}

// Write unit tests to test Caesar cipher encryption
#[cfg(test)]
mod caesar_test {

    // Access all codes in above
    use crate::Caesar;
    use crate::Cipher;

    #[test]
    fn check_caesar_pass() {
        let mut c = Caesar::default();
        c.set_shift(7);

        // Test happy case
        let result = c.encrypt("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        let expected = String::from("HIJKLMNOPQRSTUVWXYZABCDEFG");
        assert!(result.is_some());
        assert_eq!(result, Some(expected));
    }
}

fn main() {
    println!("Hello, world!");

    let mut c = Caesar::default();
    c.set_shift(4);
    let encrypted = c.encrypt("ATTACKATONCE");
    println!("Encrypted message: {:?}", encrypted);
}
