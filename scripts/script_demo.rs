//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; mkdir -p ${dst}; rustc -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

fn main() {
    println!("Hello, rust scripting.");
}
