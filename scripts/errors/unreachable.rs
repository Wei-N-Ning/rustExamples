//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

fn foobar(x: i32) {
    match x {
        n if n > 0 => println!("{}", n),
        _ => unreachable!(),
    }
}

fn main() {
    foobar(1);

    // RUST_BACKTRACE=1
    foobar(-123);
}