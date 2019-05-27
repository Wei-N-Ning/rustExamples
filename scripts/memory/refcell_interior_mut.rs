//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

use std::rc::Rc;
use std::cell::RefCell;

// algorithms with rust L703
// RefCell.RefCell maintains single ownership of a value but 
// allows mutable borrowing checked at runtime
// instead of compiler errors, violating the rules of borrowing
// will lead to a runtime panic!() crashing the program

fn demo_interior_mutability() {
    let s = Rc::new(RefCell::new(String::from("there is")));

    // immutable borrow
    println!("{}", s.borrow());
    // L719
    // this mutable reference only lives as long as the function
    // call takes (push_str()), 
    // thereby ruling out creating too-large scope and violating
    // the borrowing rules
    s.borrow_mut().push_str(" a cow !");

    // immutable borrow
    println!("{}", s.borrow());
}

fn main() {
    demo_interior_mutability();
}