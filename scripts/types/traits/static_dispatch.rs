//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

trait Presenter {
    // the type of self is statically decided
    fn present(&self) -> String;
}

impl Presenter for u32 {
    // type of self is u32
    fn present(&self) -> String {
        format!("u32({})", *self)
    }
}

impl Presenter for String {
    // type of self is String
    fn present(&self) -> String {
        format!("String({})", *self)
    }
}

fn present<T: Presenter>(x: T) {
    println!("{}", x.present());
}

fn present_dyn(x: &Presenter) {
    println!("{}", x.present());
}

fn main() {
    // static dispatch: the type of the parameter is known at 
    // compile time
    // compiler chooses the right implementation and inline it 
    present(12);
    present(String::new());
    present("there is a cow".to_string());

    // dynamic dispatch:
    // note the function parameter uses trait instead of generic 
    // type T;
    // note the use a pointer-like idiom (otherwise compiler 
    // can not figure out the size of the object)
    let x: &Presenter = &42;
    let y = &"there is a silence".to_string() as &Presenter;
    present_dyn(x);
    present_dyn(y);
}

