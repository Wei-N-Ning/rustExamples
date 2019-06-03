//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// https://doc.rust-lang.org/std/option/enum.Option.html#method.take

fn demo_take_value() {
    // saw this kind of use of take() in algorithm with Rust book
    // L1482; the resulting value from take() is piped into map()
    let v = Some(32).take().map(|v| { 
        println!("inside map(): {:?}", v);
        v
    });
    // note: the return value is STILL a std::option::Option
    // must use it in a if let or match statement
    // the example in the book L1482 uses the same pattern, 
    // returning an Option<String>
    if let Some(n) = v {
        println!("as a result: {:?}", n);
    }
}

fn demo_take_none() {
    // for None, the map part is not called at all
    // which makes it safe to run the code in the book's example
    // (while the singly linked list's head is not None, chop
    // the head but it will stop when the head becomes None)
    let v = None.take().map(|v : i32| {
        println!("inside map() (should not be called): {:?}", v);
    });
    println!("as a result (should be None): {:?}", v);
}

fn main() {
    demo_take_value();
    demo_take_none();
}
