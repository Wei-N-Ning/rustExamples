//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

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
        // iterate over each arg
        $(
            print!("{}", $x);
        )*
        println!("");
    }
}

fn main() {
    say!();
    say!(1);
}
