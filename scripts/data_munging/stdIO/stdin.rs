//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

use std::io;

fn demo_take_input() {
    let mut text = String::new();
    while io::stdin().read_line(&mut text).unwrap() > 1 {
        print!("input: {}", text);
        text.clear();
    }
    println!("+ done");
}

fn main() {
    demo_take_input();
}