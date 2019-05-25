//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

fn main() {
    let t: (i32, char) = (21, 'a');
    // tuple elements are accessed via dot
    println!("{} {}", t.0, t.1);
}