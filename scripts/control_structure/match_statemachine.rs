//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// sources

// char case-switching
// https://doc.rust-lang.org/std/primitive.char.html#method.to_uppercase
// see also: https://stackoverflow.com/questions/35432199/convert-a-char-to-upper-case
// because it returns an iterator - how to roll the iterator and 
// take the first return value? (.next().unwrap())
// https://doc.rust-lang.org/std/iter/fn.once.html

// also find this
// https://danielkeep.github.io/itercheat_baked.html

// how to modify the enum mut reference in a match expression?
// found this (not helpful) https://stackoverflow.com/questions/36928569/enums-with-constant-values-in-rust
// did this: *st = Enum

// example of usage
// ./match_statemachine.rs.bin '^there ias caow^ #comment #_DOO# aad  #M3 193_ID_'

use std::env;

enum State {
    Upper,
    Lower,
    Normal,
    Comment,
}

fn process_char(c: char, st: &mut State) -> char {
    match c {
        '^' => {
            match st {
                State::Upper => *st = State::Normal,
                State::Normal => *st = State::Upper,
                _ => {},
            };
            '\0'
        },
        '_' => {
            match st {
                State::Lower => *st = State::Normal,
                State::Normal => *st = State::Lower,
                _ => {},
            };
            '\0'
        },
        '#' => {
            match st {
                State::Comment => *st = State::Normal,
                State::Normal => *st = State::Comment,
                _ => {},
            };
            '\0'
        },
        ch => {
            match st {
                State::Upper => c.to_uppercase().next().unwrap(),
                State::Lower => c.to_lowercase().next().unwrap(),
                _ => '\0',
            }
        }
    }
}

fn process_text(s: &String) -> String {
    let mut t = String::new();
    let mut state = State::Normal;
    for c in s.chars() {
        match process_char(c, &mut state) {
            '\0' => {},
            ch => t.push(ch),
        };
    }
    t
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let t = process_text(&args[1]);
            println!("{}", t);
        },
        _ => {
            println!("prog <text>");
        }
    } 
}