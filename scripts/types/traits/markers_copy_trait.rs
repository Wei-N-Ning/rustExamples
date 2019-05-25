//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// markers are anti-traits
// to inform the compiler

#[derive(Debug)]
struct Param {
    x: i32
}

// copy is trivial bitwise copy; can not be overloaded
// it is a marker trait (no implementation body)
// note that Copy is not enough
// the trait `std::clone::Clone` is not implemented for `Param`
impl Copy for Param {}

impl Clone for Param {
    fn clone(&self) -> Param {
        *self
    }
}

// compiler to implement the Copy and Clone trait for this 
// struct
#[derive(Debug, Copy, Clone)]
struct SimpleParam {}

// Other useful markers:
// Send: safe to send to another thread
// Sync: safe to share between threads

fn main() {
    let param = Param { x: 1 };
    let a = param;
    // without copy trait: value used here after move
    println!("{:?} {:?}", a, param);   

    let p = SimpleParam {};
    let b = p;
    println!("{:?} {:?}", b, p);
}
