//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin;
//$(which mkdir) -p ${dst};
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

use std::cell::RefCell;
use std::rc::Rc;

// algorithms with rust L1482
type NodeType = Rc<RefCell<Node>>;
type SingleLink = Option<NodeType>;

#[derive(Debug)]
struct Node {
    value: String,
    // heap alloc, pointer size, let the compiler to decide
    // the size of this struct
    next: SingleLink,
}

impl Node {
    fn new(value: String) -> NodeType {
        Rc::new(RefCell::new(Node {
            value: value.clone(),
            next: None,
        }))
    }
}

#[derive(Debug)]
struct TransactionLog {
    head: SingleLink,
    tail: SingleLink,
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

    // take advantage of Option.take(), and use match statement
    // https://doc.rust-lang.org/std/option/enum.Option.html#method.take
    // Takes the value out of the option, leaving a None in its place.
    // see also: https://doc.rust-lang.org/stable/std/mem/fn.replace.html
    //
    pub fn append(&mut self, value: String) {
        let n = Node::new(value);
        match self.tail.take() {
            // Some(old), old is of NodeType, which is Rc<RefCell<T>>
            // must call borrow_mut() to create short-living mut ref
            // can also use ref pattern here, old is an immut
            // ref to Rc<RefCell<T>>; it does not bring a big benefit
            Some(old) => old.borrow_mut().next = Some(n.clone()),

            // self.head is of Option type therefore can directly
            // take Some(n)
            None => self.head = Some(n.clone()),
        };
        self.length += 1;
        self.tail = Some(n);
    }

    // compare this to the example in the book
    pub fn _pop(&mut self) -> Option<String> {
        match self.head.take() {
            Some(old) => {
                let n = old.borrow_mut();
                let s = n.value.clone();
                self.head = n.next.clone();
                self.length -= 1;
                if self.length == 0 {
                    self.tail.take();
                }
                return Some(s);
            }
            None => {
                self.tail.take();
                return None;
            }
        };
    }

    // example from book L1482
    pub fn pop2(&mut self) -> Option<String> {
        // map will naturally stop when head is None
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                // chop the head
                self.head = Some(next);
            } else {
                // no head anymore, chop the tail
                self.tail.take();
            }
            // if map() stops calling the inner block, length
            // won't be decremented any more
            self.length -= 1;

            // piping the String out
            Rc::try_unwrap(head)
                .ok()
                .expect("Something is terribly wrong")
                .into_inner() // consumes the refcell
                .value // returns an option!
        })
    }

    pub fn drop(&mut self) {
        while !self.tail.is_none() {
            self.pop2();
        }
    }
}

pub struct LogIterator {
    current: SingleLink,
}

impl LogIterator {
    fn new(start_at: SingleLink) -> LogIterator {
        LogIterator { current: start_at }
    }
}

impl Iterator for LogIterator {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        let current_link = self.current.clone();
        let mut result = None;
        self.current = match current_link {
            // I can also use value semantics here, instead of 
            // using an immut ref which saves a call to clone
            // the Rc; but isn't clone() the suggested approach?
            Some(current_ptr) => {
                let current_node = current_ptr.borrow();
                result = Some(current_node.value.clone());
                current_node.next.clone()
            },
            None => None
        };
        result
    }
}

fn test_list_creation() {
    let translog = TransactionLog::new_empty();
    let head = Node::new("e1m1".to_string());
    assert_eq!(translog.length, 0);
    assert!(head.borrow().next.is_none());
}

fn test_list_append() {
    let mut translog = TransactionLog::new_empty();
    // range is exclusive!!
    (1..5).for_each(|idx| {
        translog.append("AA_".to_string() + &idx.to_string());
    });
    assert_eq!(translog.length, 4);
}

fn test_list_pop() {
    let mut translog = TransactionLog::new_empty();
    (1..5).for_each(|idx| {
        translog.append("AA_".to_string() + &idx.to_string());
    });
    (1..10).for_each(|_| {
        if let Some(s) = translog.pop2() {
            print!("popped: {}, ", s);
        }
    });
    println!("");
    assert_eq!(translog.length, 0);
}

fn test_list_drop() {
    let mut translog = TransactionLog::new_empty();
    (1..5).for_each(|idx| {
        translog.append("AA_".to_string() + &idx.to_string());
    });
    // default drop() impl is recursive, so is Debug print trait
    // use a custom iterative impl
    translog.drop();
    assert_eq!(translog.length, 0);
    assert!(translog.tail.is_none());
    assert!(translog.head.is_none());
}

fn test_log_iterator() {
    let mut translog = TransactionLog::new_empty();
    (0..10).for_each(|idx| {
        translog.append("AA_".to_string() + &idx.to_string());
    });
    let iter = LogIterator::new(translog.head);
    iter.for_each( |s| print!("{}, ", s) );
    println!("");
}

fn main() {
    test_list_creation();
    test_list_append();
    test_list_pop();
    test_list_drop();
    test_log_iterator();
}
