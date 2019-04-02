//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

fn main() {
    // only run in un-optimized builds
    // to experiment, add -O (optimize) in the rustc commandline 
    // above then this line won't be executed (it is not in the 
    // binary) - the exitcode is 0
    debug_assert!(1 == 2);
}