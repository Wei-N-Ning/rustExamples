//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};

fn demo_channels() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = channel();
    let handles = (0..10).map(|i| {
        let _tx = tx.clone();
        thread::spawn(move || {
            // don't use the result
            let _ = _tx.send(i).unwrap();
        })
    });

    // close all threads
    for h in handles {
        h.join().unwrap();
    }

    // receive N times
    let numbers: Vec<i32> = (0..10).map(|_|
        rx.recv().unwrap()
    ).collect();

    println!("{:?}", numbers);
}

fn main() {
    demo_channels();
}