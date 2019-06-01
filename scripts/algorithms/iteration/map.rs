//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// recall map {} in perl and ruby
fn iterator_map() {
    let nums = vec![3, 1, 4, 1, 5, 9];
    let operand = 1;
    let plus_one = nums.iter().map(|x| x + operand);
    // map() is a closure; it can use the variables in the 
    // calling scope; same applies to filter()
    plus_one.for_each(|x| print!("{}, ", x));
    println!("");

    // the original vector is unmodified
    nums.iter().for_each(|elem| print!("{}, ", elem));
    println!("");
}

fn main() {
    iterator_map();
}