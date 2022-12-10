use std::process::exit;

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
                *char = (*char as u32 + self.shift - 65) % 26 + 65;
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
        let uppercase_enc = enc.to_uppercase();
        let mut chars: Vec<u32> = uppercase_enc.chars().map(|c| c as u32).collect();
        for char in &mut chars {
            if *char != (' ' as u32) {
                *char = *char + 65; // Add 65 to rotate upwards by 1 round to avoid subtract overflow
                *char = (*char as u32 - self.shift as u32) % 26 + 65;
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
}

// Write unit tests to test Caesar cipher encryption
#[cfg(test)]
mod caesar_test {

    // Access all codes in above
    use crate::Caesar;
    use crate::Cipher;

    #[test]
    fn check_caesar_enc_pass() {
        let mut c = Caesar::default();
        c.set_shift(7);

        // Test happy case
        let result = c.encrypt("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        let expected = String::from("HIJKLMNOPQRSTUVWXYZABCDEFG");
        assert!(result.is_some());
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn check_caesar_dec_pass() {
        let mut c = Caesar::default();
        c.set_shift(7);

        // Test happy case
        let expected = c.decrypt("HIJKLMNOPQRSTUVWXYZABCDEFG");
        let result = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        assert!(expected.is_some());
        assert_eq!(expected, Some(result));
    }
}

fn main() {
    println!("Hello, world!");

    let mut c = Caesar::default();
    c.set_shift(4);

    let now_enc = std::time::Instant::now();
    let encrypted = c.encrypt("ATTACKATONCE");
    let encrypted_message: String;
    match encrypted {
        Some(e) => {
            encrypted_message = e.clone();
            println!("Encrypted message: {:?}", e);
        }
        _ => {
            println!("Failed to encrypt");
            exit(-1);
        }
    }
    println!(
        "Encrypted time elapsed: {:?} us",
        now_enc.elapsed().as_micros()
    );

    let now_dec = std::time::Instant::now();
    let decrypted = c.decrypt(&encrypted_message);
    match decrypted {
        Some(d) => {
            println!("Decrypted message: {:?}", d);
        }
        _ => {
            println!("Failed to dencrypt");
            exit(-1);
        }
    }
    println!(
        "Decrypted time elapsed: {:?} us",
        now_dec.elapsed().as_micros()
    );
}
