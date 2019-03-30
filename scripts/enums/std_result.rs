//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// std::result
// recall golang's -> (err, result) idiom
fn get_middle(elements: Vec<i32>) -> Result<i32, &'static str> {
    match elements.len() {
        0 => Err("empty"),
        x if x % 2 == 0 => Err("even number of elements"),
        _ => Ok(elements[elements.len() / 2]),
    }
}

fn main() {
    println!("{:?}", get_middle(vec![]));
    println!("{:?}", get_middle(vec![1, 2, 3]));
    println!("{:?}", get_middle(vec![1, 2]));

    // see https://doc.rust-lang.org/std/result/
    // for std::result match syntax
    match get_middle(vec![]) {
        Err(e) => println!("fail: {}", e),
        Ok(v) => println!("success: {}", v),
    }
    match get_middle(vec![4, 5, 6]) {
        Err(e) => println!("fail: {}", e),
        Ok(v) => println!("success: {}", v),
    }
}
