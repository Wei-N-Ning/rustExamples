//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

const FIB_ZERO: u64 = 0;

fn main() {
    // invalid left hand expression
    // FIB_ZERO = 1;

    println!("{}", FIB_ZERO);
}