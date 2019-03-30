//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// see also standard library's Option type
enum Option<T> {
    Some(T),
    None,
}

fn test_option() {
    let some_i = Option::Some(42);
    let some_f = Option::Some(3.14);
    let none: Option<i32> = Option::None;

}

fn main() {
    test_option();
}
