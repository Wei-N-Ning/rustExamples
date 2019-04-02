//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

trait Weapon {
    fn kill(&self);
}

struct Gun {}
impl Weapon for Gun {
    fn kill(&self) {
        unimplemented!();
    }
}

fn main() {
    let g = Gun {};
    // RUST_BACKTRACE=1 ./unimplemented.rs
    g.kill();
}