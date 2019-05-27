//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

use std::ops::Fn;

// a closure implements one of the Fn family of traits,
// meaning it can be called with () syntax, like a function
fn demo_closure_creation() {
    let clo1 = |x| {
        x + 1
    };
    println!("{}", clo1(123));
}

// we can not write the type of a function - it is called
// anonymous type of unspeakable type
// to return it or accept it in a function, we must use 
// generic or a dynamic trait object

// FnOnce -> FnMut -> Fn
// Fn: can run any number of times, only closing over immutable bindings,
// FnMut: can run any number of times, closing over mut and immut 
// FnOnce: can run once, taking ownership of captured bindings

// FnOnce implementer can be spawned into a thread 
// this is done using std::thread::spawn()
// it creates full OS threads, not green threads (goroutines...)

fn main() {
    demo_closure_creation();
}