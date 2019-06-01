//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// - closure is a function that captures its environment
// - this means it is defined inline with other code and can 
//   access bindings declared in that code
// - they are anonymous and their type can not be named (can not 
//   do "this function takes this type of closure")
// - a closure implements one of the Fn family of traits,
//   meaning it can be called with () syntax, like a function
// - to return it or accept it in a function, must use generic
//   or dynamic trait object

// source
// how to return closure
// https://doc.rust-lang.org/rust-by-example/fn/closures/output_parameters.html

// Closures as input parameters are possible, so returning closures 
// as output parameters should also be possible. However, returning 
// closure types are problematic because Rust currently only supports 
// returning concrete (non-generic) types. Anonymous closure types 
// are, by definition, unknown and so returning a closure is only 
// possible by making it concrete. This can be done via boxing.
fn demo_closure_creation() -> Box<Fn() -> i32> {
    let clo1 = |x| { x + 1 };
    apply(clo1);
    println!("called: {}", clo1(123));

    Box::new(move || { 10 })
}

// source
// how to pass closure to function
// https://doc.rust-lang.org/rust-by-example/fn/closures/input_parameters.html

// A function which takes a closure as an argument and calls it.
fn apply<F>(f: F) where
    // The closure takes no input and returns nothing.
    F: FnOnce(i32) -> i32
{
    // ^ TODO: Try changing this to `Fn` or `FnMut`.

    println!("applied: {}", f(213));
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
    let f = demo_closure_creation();
    println!("called as return value: {}", f());
}