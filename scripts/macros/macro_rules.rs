//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

//////////////////////////////////
// macros do not have namespaces!!
//////////////////////////////////

macro_rules! say {
    // matches no argument
    () => {
        // call to say!() will be expanded to 
        println!("thereisacow");
    };
    // match an expression and assign it to x
    // arguments (if many) can be separated by comma
    // *: allow repeating
    ( $( $x:expr ),* ) => {
        print!("there are: ");
        // iterate over all the arguments (repeating N times)
        $(
            print!("{}", $x);
        )*
        println!("");
    }
}

fn main() {
    say!();
    say!(1);
    say!('a', 'b', 'c');
    assert!(1 == 1);
}
