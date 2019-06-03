fn foobar() -> Option<i32> {
    Some(10)
}

fn main() {
    println!("{}", foobar().unwrap())

}
