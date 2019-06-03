//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

struct Param {
    value: i32,
}

impl From<i32> for Param {
    fn from(v: i32) -> Self {
        Param { value: v }
    }
}

fn main() {
    let param = Param::from(213);
    println!("{}", param.value);
}
