//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// behave similarly to ruby/crystal's reduce()
fn iterator_fold() {
    let nums = vec![3, 1, 4, 1, 5, 9];
    let sum:i32 = nums.iter().fold(
        0, |sum, val| sum + val
    // init accu elem
    );
    println!("Reduce-sum: {}", sum);
}

fn main() {
    iterator_fold();
}