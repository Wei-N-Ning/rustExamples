//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// source
// https://doc.rust-lang.org/std/fs/struct.File.html

use std::fs::File;
use std::io::prelude::*;

fn demo(filename: String) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(b"...")?;
    Ok(())
}

fn main() {
    demo("/var/tmp/test.txt".to_string()).expect("failed");

    /**
     * thread 'main' panicked at 'failed: Os { code: 2, kind: NotFound, 
     * message: "No such file or directory" }', src/libcore/result.rs:997:5
     * note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    */
    // demo("/vaer/wr23/423".to_string()).expect("failed");
}
