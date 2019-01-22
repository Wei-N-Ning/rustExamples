//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// without this the code won't compile
// every member in the enum must use 
// the debug technique
#[derive(Debug)]
pub struct Bed {
    size: u32,
    count: u32,
}

// use the debug print technique
#[derive(Debug)]
pub enum Room {
    Kitchen,
    Bedroom(Bed),
    Lounge(i8),  // to specify the width of the enum
}

fn main() {
    // save typing in local scope
    // no need to use Room::Lounge
    use self::Room::*;

    // enum variable is stored with minimal amount of bytes (both the type and the var)
    let mut t = Lounge(8);
    println!("{:?}", t);

    match t {
        Lounge(n) => println!("lounge of size {}", n),
        other => println!("{:?}", other),
    }

    t = Kitchen;
    let v = match t {
        Kitchen => 101,
        _ => 0,
    };
    println!("match result: {}", v);

    if let Bedroom(Bed{size:1, count:1}) = t {
        // no need to assume
        // type of t CAN ONLY BE Bedroom
        ;
    }
}