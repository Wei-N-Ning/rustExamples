//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; mkdir -p ${dst}; rustc -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
