//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

use std::env;

fn demo_read_env_var() {
    let vars = env::vars();
    for (k, v) in vars {
        println!("{}={}", k, v);
    }
}

fn main() {
    demo_read_env_var();
}