//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// Macro:

// declarative macro
// vec!(), println!()
// variadic functions
// expanded early in the compile time, with proper type checks
// static dispatch, no perf cost
// doing things that are not possible with the lang
// example: manipulate raw syntax tokens: 
//   stringify!(1 == 2) -> "1 == 2"

// procedural macro
// #[derive(Debug, Clone, Copy)]

fn main() {
    ;
}
