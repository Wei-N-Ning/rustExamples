//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

use std::io;
use std::fs::File;
use std::io::prelude::*;

fn safe_file_create(filename: String) -> Result<(), io::Error> {
    // File::create() -> Result<File>
    // try!() will try to unwrap the value and in the case of 
    // error the whole function will early-out with an error
    // object;
    // note the return type of this function is Result<>, because 
    // of the behavior of try!()
    let mut f = try!(File::create(filename));
    let _ = f.write_all(b"thereisacow");
    Ok(())
}

// try!() can be shortened to ?
fn safe_file_create_s(filename: String) -> Result<(), io::Error> {
    File::create(filename)?.write_all(b"thereisacow")
}

// equivalent to 
fn safe_file_create_match(filename: String) -> Result<(), io::Error> {
    match File::create(filename) {
        Ok(mut f) => f.write_all(b"thereisacow"),
        Err(e) => return Err(e)
    }
}

fn main() {
    let mut result = safe_file_create("/dev/you/can/not".to_string());
    println!("{:?}", result);
    result = safe_file_create("/var/tmp/foo".to_string());
    println!("{:?}", result);
    result = safe_file_create_match("/var/tmp/foo".to_string());
    println!("{:?}", result);
    result = safe_file_create_s("/dev/you/cannot".to_string());
    println!("{:?}", result);
}