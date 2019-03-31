//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

trait X {}
trait Y {}
trait Z: X + Y {}

struct A {}

// failing to impl X and Y first:
//  the trait `Y` is not implemented for `A`

impl X for A {}
impl Y for A {}
impl Z for A {}

fn main() {
    A{};
}