//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

fn demo_box() {
    // like C's malloc() and C++'s smart pointer
    // R/W access
    // RAII idiom - only used in one place, resource is deallocated
    // once it goes out of scope
    let _boxed = Box::new("data");
}

fn main() {
    demo_box();
}