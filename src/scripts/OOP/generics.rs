//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// struct that is agnostic to the types of the data member
struct Point<T> {
    x: T,
    y: T,
}

// implements the interface Point
impl<T> Point<T> {
    fn distance(&self) -> &T {
        println!("generic impl");
        &self.x
    }
}

// implements for concrete type
impl Point<i32> {
    fn idistance(&self) -> &i32 {
        println!("i32 impl");
        &self.x
    }
}

fn test_point() {
    let ipoint = Point {x:5, y:10};
    let fpoint = Point {x:5.0, y:10.0};

    // type mismatch (x's type, Float, dictates T now)
    // let fipoint = Point {x:5.0, y:10};
    // it is great that it does not implicitly convert Int to
    // Float
    println!("{:?} {:?}", ipoint.x, fpoint.y);
    println!("distance: {}", ipoint.distance());
    println!("distance: {}", ipoint.idistance());
}

fn main() {
    test_point();
}

