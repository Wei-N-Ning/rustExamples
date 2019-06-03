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

// cheap to copy
// recall modern effective c++: value object should always be copied
#[derive(Copy, Clone)]
enum State {
    Upper,
    Lower,
    Normal,
    Comment,
}

fn process_char(c: char, st: State) -> (Option<char>, State) {
    use self::State::*;

    // match state then match character - this way the comment
    // mode can be handled correctly

    // match st {
    //     Normal => match c {
    //         '#' => (None, Comment),
    //         '_' => (None, Lower),
    //         '^' => (None, Upper),
    //         other => (Some(other), Normal),  // note the use of Some()
    //     },
    //     Comment => match c {
    //         '#' => (None, Normal),
    //         other => (Some(other), Comment),
    //     },
    // }

    // match a tuple
    match (st, c) {
        (Normal, '#') => (None, Comment),
        (Normal, '^') => (None, Upper),
        (Normal, '_') => (None, Lower),
        (Normal, other) => (Some(other), Normal),  // other is a new binding
        (Comment, '#') => (None, Normal),
        (Comment, _) => (None, Comment),
        (Upper, '^') => (None, Normal),
        (Upper, other) => (Some(other.to_ascii_uppercase()), Upper),
        (Lower, '_') => (None, Normal),
        (Lower, other) => (Some(other.to_ascii_lowercase()), Lower),
    }
}

fn process_text(s: &String) -> String {
    let mut st = State::Normal;
    let mut new_s = String::new();
    for c in s.chars() {
        let (output, new_st) = process_char(c, st);

        // prefer if let expression to match expr
        // match output {
        //     None => (), 
        //     Some(new_c) => new_s.push(new_c),
        // };

        // this will throw out the None case
        if let Some(new_c) = output {
            new_s.push(new_c);
        }
        st = new_st;
    }
    return new_s;
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