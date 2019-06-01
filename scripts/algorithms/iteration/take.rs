//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

fn demo_take() {
    // source
    // https://doc.rust-lang.org/std/iter/trait.FromIterator.html
    use std::iter::FromIterator;
    let five_fives = std::iter::repeat(5).take(5);
    let v = Vec::from_iter(five_fives);
    assert_eq!(v, vec![5, 5, 5, 5, 5]);
}

fn main() {
    demo_take();
}