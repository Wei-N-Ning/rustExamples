//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// algorithms with rust L496

fn shoot(val: i32) -> Option<i32> {
    if val > 0 {
        return Some(val);
    }
    return None;
}

// only care if the return value is not None
fn demo_if_let(val: i32) {
    if let Some(o) = shoot(val) {
        println!("shoot: {}", o);
    }
}

fn main() {
    demo_if_let(1331);
    demo_if_let(-131);
}