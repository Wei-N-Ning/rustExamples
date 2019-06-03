//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// https://doc.rust-lang.org/std/option/enum.Option.html#method.take

fn demo_take() {
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

fn main() {
    demo_take();
}
