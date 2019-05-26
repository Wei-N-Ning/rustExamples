//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

fn demo_ref_counter() {
    // can R-access in many places
    let _rc = std::rc::Rc::new("data");
}

fn main() {
    demo_ref_counter();
}