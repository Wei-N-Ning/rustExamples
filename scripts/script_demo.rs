//$(which true); dst=/var/tmp/sut; mkdir -p ${dst}; rustc -o "${dst}/$0.bin" 1>&2 "$0" && "${dst}/$0.bin" "$@"; exit $?

fn main() {
    println!("Hello, rust scripting.");
}
