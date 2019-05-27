//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

use std::fmt::Display;

// use the behavior offered by the display trait and runtime,
// ignoring the type of the object
// a vector of generics 
// e.g. a vector of "trait objects"
// requires an extra pointer lookup
// the objects lose their other behavior (such as String::push)
fn show_all(v: Vec<&dyn Display>) {
    for item in v {
        println!("{}", item);
    }
}

fn main() {
    let v = vec![&12 as &Display, &"Hi" as &Display];
    show_all(v);
}
