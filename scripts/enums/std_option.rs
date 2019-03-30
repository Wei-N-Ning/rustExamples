//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// std::option
// option is useful for return type
// something that may or may not have a value
fn first_element(elements: &Vec<i32>) -> Option<i32> {
    if elements.len() == 0 {
        None
    }
    else {
        Some(elements[0])
    }
}

fn main() {
    let empty = [].to_vec();
    let nums = vec![3, 1, 4, 1, 5];

    match first_element(&empty) {
        None => println!("empty"),
        Some(x) => println!("[0]: {}", x),
    }
    match first_element(&nums) {
        None => println!("empty"),
        Some(x) => println!("[0]: {}", x),
    }
}
