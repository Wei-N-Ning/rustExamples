//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin;
//$(which mkdir) -p ${dst};
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

use std::cell::RefCell;
use std::rc::Rc;

type NodeType = Rc<RefCell<Node>>;
type Link = Option<NodeType>;

#[derive(Debug, Clone)]
struct Node {
    pub value: String,
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

    pub fn append(&mut self, _s : String) {
        ;
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

impl Iterator for ListIterator {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let current_link = self.current.clone();
        let mut result = None;
        self.current = match current_link {
            Some(ref current_cell) => {
                let current_ref = current_cell.borrow();
                result = Some(current_ref.value.clone());
                current_ref.next.clone()
            },
            None => None
        };
        result
    }
}

impl DoubleEndedIterator for ListIterator {
    fn next_back(&mut self) -> Option<String> {
        let current_link = self.current.clone();
        let mut result = None;
        self.current = match current_link {
            Some(ref current_cell) => {
                // immut ref
                let current_ref = current_cell.borrow();
                // clone the string to be returned
                result = Some(current_ref.value.clone());
                // don't want to pass the ownership from the 
                // node to the iter, hence giving it a clone
                current_ref.prev.clone()
            },
            None => None
        };
        result
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

fn test_list_append() {
    let mut l = TransactionLog::new_empty();
    l.append(String::from("AA"));
}

fn test_list_iterator() {
    let n = Node {
        value: "AA".to_string(),
        next: None,
        prev: None,
    };
    let l = Some(Rc::new(RefCell::new(n)));
    let it = ListIterator::new(l);
    it.for_each(|s| println!("{}", s));
}

fn main() {
    test_node_creation();
    test_list_append();
    test_list_iterator();
}
