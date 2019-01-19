//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; mkdir -p ${dst}; rustc -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// source
// https://doc.rust-lang.org/book/ch03-03-how-functions-work.html

fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
