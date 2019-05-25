//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// source
// 7 ways to concat strings, benchmarking
// https://github.com/hoodie/concatenation_benchmarks-rs
// https://www.reddit.com/r/rust/comments/48fmta/seven_ways_to_concatenate_strings_in_rust_the/

use std::collections::LinkedList;

fn demo_creation() {
    let mut ll = LinkedList::new();

    // without this compiler can not infer the type of ll
    (1..10).for_each(|x| ll.push_back(x));
    // without : String the resulting value's type can not be 
    // inferred
    let s: String = ll.iter().map(|elem| format!("{} ", elem)).collect();
    println!("{}", s);
}

fn main() {
    demo_creation();
}