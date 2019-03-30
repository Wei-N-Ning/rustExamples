//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// struct that is agnostic to the types of the data member
struct Point<T> {
    x: T,
    y: T,
}

fn test_point() {
    let ipoint = Point {x:5, y:10};
    let fpoint = Point {x:5.0, y:10.0};

    // type mismatch (x's type, Float, dictates T now)
    // let fipoint = Point {x:5.0, y:10};
    // it is great that it does not implicitly convert Int to
    // Float
    println!("{:?} {:?}", ipoint.x, fpoint.y);
}

fn main() {
    test_point();
}

