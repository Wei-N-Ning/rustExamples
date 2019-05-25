//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

extern crate proc_macro;
use proc_macro::TokenStream;

trait Firearm {
    fn shoot(&self);
}

struct Rifle {}
impl Firearm for Rifle {
    fn shoot(&self) {
        println!("bang!");
    }
}

// #[derive(FirearmMacro)]
// this is equivalent to the above code, provided that the macro 
// writes out the implementation 
struct Machinegun {}

fn main() {
    let r = Rifle {};
    r.shoot();
}

