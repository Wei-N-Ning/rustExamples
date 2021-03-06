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

fn return_string() -> Option<String> {
    // map() wrap the raw value (string asd) in a Some()
    Some(1).map(|_| "asd".to_string())
}

fn _return_none() -> Option<String> {
    // this does not work: expect the raw value to be String 
    // not None
    // this function has to explicitly return None
    // Some(1).map(|_| None)
    None
}

fn demo_return_string() {
    println!("{:?}", return_string());
}

// beside using value semantics, I can use ref and mut ref
// semantics in the match statement: 
// if v is not None, and it is mutable, create a mutable ref
// for it to be used by the inner block
fn demo_ref_pattern() {
    let mut v = Some(123);
    match v {
        Some(ref mut v_ref) => {
            *v_ref += 12123;
            println!("{}", v_ref);
        },
        None => {}
    }
}

// TODO: look into option ? (and recall Haskell's way of dealing with this)

fn main() {
    demo_take_value();
    demo_take_none();
    demo_return_string();
    demo_ref_pattern();
}
