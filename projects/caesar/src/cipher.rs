extern crate rand;
use rand::prelude::*;

// rand
// https://rust-random.github.io/book/overview.html

// source: how to concat string (and how to  append char)
// https://stackoverflow.com/questions/30154541/how-do-i-concatenate-strings
pub fn cipher(cleartext: &String, key: u64) -> String {
    let mut enctext = String::new();
    let deviation = random::<u64>();
    for c in cleartext.chars() {
        enctext.push((c as u64 + key + deviation) as u8 as char);
    }
    enctext
}
