//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

fn add(lhs: i32, rhs: i64) -> i32 {
    // recall crystall use .as(<TYPE>) to cast
    lhs + rhs as i32
}

fn main() {
    println!("{} {} {}", 1, 2, add(1, 2 as i64))
}
