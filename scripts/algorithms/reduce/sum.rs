//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

fn iterator_sum() {
    let nums = vec![3, 1, 4, 1, 5, 9];
    let sum:i32 = nums.iter().sum();
    println!("Sum: {}", sum);
}

fn main() {
    iterator_sum();
}