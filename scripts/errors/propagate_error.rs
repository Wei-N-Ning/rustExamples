//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// source
// see how to throw a simple "error" (static string)
// https://doc.rust-lang.org/std/result/

// read also:
// define custom error struct
// https://doc.rust-lang.org/1.28.0/std/error/trait.Error.html
// https://stevedonovan.github.io/rust-gentle-intro/6-error-handling.html

fn can_fail(v : i32) -> Result<i32, &'static str> {
    if v > 0 {
        Ok(1)
    }
    else {
        Err("invalid")
    }
}

fn process(v : i32) -> Result<i32, &'static str> {
    Ok(v)
}

fn caller(v : i32) -> Result<i32, &'static str> {
    process(can_fail(v)?)
}

fn main() {
    [1, -1].iter().for_each( |elem| {
        match caller(*elem) {
            Ok(v) => { println!("ok: {}", v) }
            Err(err) => { println!("err: {}", err) }
        }
    });
}