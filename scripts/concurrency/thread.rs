//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// algorithms with rust L674
// concurrency and parallelism are two different modes of 
// execution
// while concurrency means that parts of a program run 
// independently of each other, parallelism refers to these 
// parts executing at the same time

use std::thread;

// || is the space where parameters go
// akin to a function signature's parameters, without the need
// to always declare types explicitly
// this way variables can move from the outer into the inner scope 
fn demo_thread_creation_join() {
    let handle = thread::spawn(|| {
        println!("threaded");
        return 1334;
    });
    if let Ok(o) = handle.join() {
        println!("joined: {:?}", o);  // <- return value from the subroutine
    }
}

fn main() {
    demo_thread_creation_join();
}