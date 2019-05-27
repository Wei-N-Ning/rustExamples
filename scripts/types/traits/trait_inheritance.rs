//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// algorithms with rust L5051
// what are traits and how are they different from interfaces
// traits are pieces of functionality shared across components,
// they can contain code as well as associated types and can 
// be implemented for any type and generics independently
// interfaces describe the public methods a class provides, 
// without an implementation and typically with inheritance

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