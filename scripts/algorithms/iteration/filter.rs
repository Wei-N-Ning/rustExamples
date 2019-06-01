//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// recall grep {} in perl and ruby
fn iterator_filter() {
    let nums = vec![3, 1, 4, 1, 5, 9];
    let larger_then_three = nums.into_iter().filter(|&x| x > 3);
    larger_then_three.for_each(|elem| print!("{}, ", elem));
    println!("");
}

fn main() {
    iterator_filter();
}