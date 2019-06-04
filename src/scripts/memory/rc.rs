//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// C++ std::shared_ptr<>
use std::rc::Rc;

fn demo_rc_creation() {
    let mut s = String::from("thereisacow");
    {
        let ptr1 = Rc::new(s); // the content of s is now owned by ptr1
                               // put the content is immut due to 
                               // multiple borrowing rule
        let ptr2 = ptr1.clone(); // increment ptr1 counter
        println!("{}, {}", ptr1, ptr2);
    }
}

fn main() {
    demo_rc_creation();
}