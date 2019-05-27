//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

fn demo_closure_creation() {
    let clo1 = |x| {
        x + 1
    };
    println!("{}", clo1(123));
}

fn main() {
    demo_closure_creation();
}