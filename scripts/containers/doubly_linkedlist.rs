//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin;
//$(which mkdir) -p ${dst};
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

use std::cell::RefCell;
use std::rc::Rc;

type NodeType = Rc<RefCell<Node>>;
type Link = Option<NodeType>;

#[derive(Debug, Clone)]
struct Node {
    value: String,
    next: Link,
    prev: Link,
}

#[derive(Debug, Clone)]
pub struct TransactionLog {
    head: Link,
    tail: Link,
    pub length: u64,
}

impl TransactionLog {
    pub fn new_empty() -> TransactionLog {
        TransactionLog {
            head: None,
            tail: None,
            length: 0,
        }
    }
}

// algorithms with rust L1555
// looking at the list without consuming it is an iterator's job
// implement the trait
pub struct ListIterator {
    current: Link,
}

impl ListIterator {
    fn new(start_at: Link) -> ListIterator {
        ListIterator { current: start_at }
    }
}

fn test_node_creation() {
    let n = Node {
        value: "AA".to_string(),
        next: None,
        prev: None,
    };
    println!("{:?}", n);
    assert!(n.next.is_none() && n.prev.is_none());
}

fn main() {
    test_node_creation();
}
