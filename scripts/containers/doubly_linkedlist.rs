//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

use std::rc::Rc;
use std::cell::RefCell;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug, Clone)]
struct Node {
    value : String,
    next : Link,
    prev : Link,
}

fn test_node_creation() {
    let n = Node { value: "AA".to_string(), next: None, prev: None };
    println!("{:?}", n);
    assert!(n.next.is_none() && n.prev.is_none());
}

fn main() {
    test_node_creation();
}