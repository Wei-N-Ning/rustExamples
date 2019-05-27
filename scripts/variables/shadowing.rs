//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// shadowing does not replace mutability
fn sanitize(s: String) -> String {
    let s = s.trim();
    let s = s.replace("_", " ");
    s
}

fn demo_shadowing() {
    let s = sanitize(String::from("there_is_a"));
    println!("{}", s);
}

fn main() {
    demo_shadowing();
}
