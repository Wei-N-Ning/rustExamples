//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// clone allows a allow to be explicitly cloned, such as String
// clone makes the compiler do it automatically

struct Item {
    x: String,
}

impl Clone for Item {
    fn clone(&self) -> Item {
        Item {x: self.x.clone()}
    } 
}

fn demo_clone() {
    let i1 = Item {x: String::from("thereisacow")};
    let i2 = i1.clone();
    println!("{} {}", i1.x, i2.x);
}

fn main() {
    demo_clone();
}