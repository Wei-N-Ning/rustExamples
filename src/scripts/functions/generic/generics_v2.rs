//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// generic functions reduce code duplication
// they take any type that implements some set of traits, and
// they can only use the behavior of those traits

// take any objects that implement the Display trait
// compiler will enforce the trait compatibility
// trait bound specifies what traits must be available
fn _print_any<T: Display>(t: T) {
    println!("{}", t);
}

fn main() {
    
    ;
}