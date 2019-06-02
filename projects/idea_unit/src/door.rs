// source
// algorithms with rust L473
// this pattern is very common in the standard library, and 
// often third-party libraries will even add behavior to 
// existing types by implementing traits in their code
// also known as extension traits
// recall Rayon's extension to iterators

struct Door {
    is_open: bool
}

impl Door {
    fn new(is_open: bool) -> Door {
        Door { is_open: is_open }
    }
}

// L480
// other than a typical class where data fields and methods are 
// in a single construct, rust emphasizes the separation between 
// thoes by declaring a struct for data and an impl part for the
// methods/functions
// traits name and encapsulate behaviors so that they can easily 
// be imported, shared and reused
trait Openable {
    fn open(&mut self);
}

impl Openable for Door {
    fn open(&mut self) {
        self.is_open = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_door() {
        let mut door = Door::new(false);
        door.open();
        assert!(door.is_open);
    }
}
