//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

use std::fmt;

trait Generator {
    type Item: fmt::Display;  
    // associated type with a trait bound:
    // item must satisfy fmt::Display

    // return associated type
    fn gen(&mut self) -> Option<Self::Item>;
}

struct StopMotion {}
impl Generator for StopMotion {
    type Item = i32;

    fn gen(&mut self) -> Option<Self::Item> {
        Some(101)
    }
}

// in one concrete type, can not implement the same method 
// using multiple Item types - compile error

struct LocoMotion {}
impl Generator for LocoMotion {
    type Item = f64;
    
    fn gen(&mut self) -> Option<Self::Item> {
        Some(3.14)
    }
}

fn main() {
    println!("stop motion: {:?}", StopMotion{}.gen());
    println!("locomotion: {:?}", LocoMotion{}.gen());
}
