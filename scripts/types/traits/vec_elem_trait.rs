//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// read:

// method has no receiver??
// https://stackoverflow.com/questions/38159771/why-can-a-trait-not-construct-itself

// can not convert to an object??
// https://stackoverflow.com/questions/45116984/the-trait-cannot-be-made-into-an-object

// WTF is box?
// https://doc.rust-lang.org/std/boxed/struct.Box.html

trait Foo {
    // all the trait methods must have receiver &self
    fn give_damage(&self) -> i32;
    fn splash_damage(&self) -> i32;

    // example of a method without receiver is a (static)
    // factory method
}

struct FooCow {}
impl Foo for FooCow {
    fn give_damage(&self) -> i32 {
        5
    }
    fn splash_damage(&self) -> i32 {
        1
    }
}

struct FooSkeleton {}
impl Foo for FooSkeleton {
    fn give_damage(&self) -> i32 {
        10
    }
    fn splash_damage(&self) -> i32 {
        3
    }
}

// takes a vector of pointers; note the trait-of-element syntax;
// this won't work Vec<Foo> because compiler can not deduce the 
// size of each element
fn damages(foos: Vec<Box<Foo>>) -> i32 {
    foos.iter().fold(
        0, |sum, foo| sum + foo.give_damage() + foo.splash_damage()
    )
}

fn main() {
    // constructs a vector of pointers
    let foos: Vec<Box<Foo>> = vec![
        Box::new(FooCow{}),
        Box::new(FooSkeleton{}),
        Box::new(FooSkeleton{}),
    ];
    let result = damages(foos);
    println!("damages: {}", result);
}
