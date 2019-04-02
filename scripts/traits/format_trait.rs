//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

use std::fmt::{Display, Formatter, Result};

#[derive(Default, Debug)]
// Default trait: generate default values for data members
struct Printable {
    x: i32,
}

impl Display for Printable {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, 
            "there is a {}, at {}", 
            "cow", 
            "ad"
        )
    }
}

fn main() {
    // calling Printable {} won't work: missing `x`
    let pp = Printable::default();  
    // Display trait, invoking fmt()
    println!("{}", pp);
    
    // Debug ..
    println!("{:?}", pp);
    // `Printable` cannot be formatted using `{:?}`
    // automatically created by compiler (#[derive...])

    // Debug, pretty-print: line indentation etc...
    println!("{:#?}", pp);
}
