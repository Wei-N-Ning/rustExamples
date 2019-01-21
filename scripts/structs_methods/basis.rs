//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// to use this struct outside the project:
// pub struct {}
#[derive(Debug)]
struct Item {
    name: String,
    size: u32,
}

fn main() {
    let item = Item {
        name: String::from("øπ"),
        size: 213,
    };
    println!("{:?}", item);
}