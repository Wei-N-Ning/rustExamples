//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

use std::cell::RefCell;
use std::rc::Rc;

// algorithms with rust L5240
type NodeType = Rc<RefCell<Node>>;
type SingleLink = Option<NodeType>;

#[derive(Debug)]
struct Node {
    value : String,
    // heap alloc, pointer size, let the compiler to decide
    // the size of this struct
    next : SingleLink,
}

impl Node {
    fn new(value : String) -> NodeType {
        Rc::new(RefCell::new(Node {
            value: value.clone(),
            next: None,
        }))
    }
}

#[derive(Debug)]
struct TransactionLog {
    head : SingleLink,
    tail : SingleLink,
    pub length : u64,
}

impl TransactionLog {
    pub fn new_empty() -> TransactionLog {
        TransactionLog { head: None, tail: None, length: 0 }
    }

    // take advantage of Option.take(), and use match statement
    // https://doc.rust-lang.org/std/option/enum.Option.html#method.take
    // Takes the value out of the option, leaving a None in its place.  
    // see also: https://doc.rust-lang.org/stable/std/mem/fn.replace.html
    // 
    pub fn append(&mut self, value : String) {
        let n = Node::new(value);
        match self.tail.take() {
            // Some(old), old is of NodeType, which is Rc<RefCell<T>>
            // must call borrow_mut() to create short-living mut ref
            Some(old) => old.borrow_mut().next = Some(n.clone()),

            // self.head is of Option type therefore can directly
            // take Some(n)
            None => self.head = Some(n.clone())
        };
        self.length += 1;
        self.tail = Some(n);
    }

    pub fn remove(&mut self) {
        match self.head.take() {
            Some(old) => {
                let n = old.borrow_mut();
                self.head = n.next.clone();
                self.length -= 1;
            }
            None => {
                self.tail.take();
            }
        };
    }
}

fn test_list_creation() {
    let translog = TransactionLog::new_empty();
    let head = Node::new("e1m1".to_string());
    println!("{:?} {:?}", translog, head);
}

fn test_list_append() {
    let mut translog = TransactionLog::new_empty();
    // range is exclusive!!
    (1..3).for_each(|_| {
        translog.append("AA".to_string());
    });
    println!("{:?}", translog);
    assert_eq!(translog.length, 2);
}

fn test_list_remove() {
    let mut translog = TransactionLog::new_empty();
    (1..3).for_each(|_| {
        translog.append("AA".to_string());
    });
    (1..5).for_each(|_| {
        translog.remove();
    });
    println!("{:?}", translog);
    assert_eq!(translog.length, 0);
}

fn main() {
    test_list_creation();
    test_list_append();
    test_list_remove();
}