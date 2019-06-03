//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// to use this struct outside the project:
// pub struct {}

// add this: #[derive(Debug)]
// then the object can be converted to string (for debugging)
// {:?}

struct Item {
    name: String,
    size: u32,
}

impl Item {
    // self: object
    // &self: const pointer
    // &mut self: pointer
    fn to_string(&self) -> String {
        format!("<{}, {}>", self.name, self.size)
    }

    fn set_size(&mut self, size: u32) {
        self.size = size;
    }

    // consumer method use move semantic to "consume" the
    // object passed in
    fn destroy(self) {
    } 
}

fn main() {
    let mut item = Item {
        name: String::from("øπ"),
        size: 213,
    };
    item.set_size(123);
    println!("{}", item.to_string());

    item.destroy();
    // item.set_size(456);  // compiler: value used here after move
}


