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

// adding a couple of long notes after watching this episode 
// again

// this sounds like C++ template

// "hey compiler, this function can handle all kinds of types,
// but I do not want to write them out, so just figure them out
// at compile time and generate each version of this function for
// each of those types"

// this sounds like C++ vtable-based inheritance

// a generic type call &dyn Display; &: reference; Display is 
// a trait; dyn means to do dynamic dispatch here, with the 
// performance penalty; "this is not a generic, but any types 
// that implements the Display trait"; compiler creates a vtable
// and a pointer to it
// the heterogeneous objects are created by the caller (the 
// creator of the vector of Display trait implementer, the so 
// called "trait objects"), this uses the "as" technique to 
// dynamic_cast<> various objects, such as i32, String etc.
// to the trait objects; as with dynamic_cast, the trait object 
// no longer behaves like the origin copy but its data is still
// there
// function_call -> argument (ptr) -> vtable -> function
fn show_all(v: Vec<&dyn Display>) {
    for item in v {
        println!("{}", item);
    }
}

fn main() {
    let v = vec![&12 as &Display, &"Hi" as &Display];
    show_all(v);
}
