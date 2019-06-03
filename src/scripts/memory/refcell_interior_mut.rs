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

    // immutable borrow (immut ref)
    assert_eq!(*s.borrow(), "there is");
    // L719
    // this mutable reference only lives as long as the function
    // call takes (push_str()), 
    // thereby ruling out creating too-large scope and violating
    // the borrowing rules
    s.borrow_mut().push_str(" a cow !");

    // immutable borrow (returns an immutable reference)
    assert_eq!(*s.borrow(), "there is a cow !");
}

fn demo_into_inner() {
    let cell = RefCell::new(5);
    let v1 = cell.into_inner();
    // 32 |     let v1 = cell.into_inner();
    //    |              ---- value moved here
    // 33 |     let v2 = cell.into_inner();
    //    |              ^^^^ value used here after move
    // let v2 = cell.into_inner();
    assert_eq!(v1, 5);
}

fn main() {
    demo_interior_mutability();
    demo_into_inner();
}
