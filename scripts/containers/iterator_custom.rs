//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// source
// https://www.udemy.com/rust-building-reusable-code-with-rust-from-scratch/learn/lecture/13316098#overview

struct CreStore {
    count :i32,
}

impl Iterator for CreStore {
    type Item = i32;
    fn next(&mut self) -> std::option::Option<i32> {
        if self.count < 10 {
            let v = self.count;
            self.count += 1;
            return Some(v);
        } 
        else {
            return None;
        }
    }
}

fn main() {
    let mut store = CreStore { count: 6 };
    while let Some(v) = store.next() {
        println!("{}", v);
    }
    // store continues to return None
}