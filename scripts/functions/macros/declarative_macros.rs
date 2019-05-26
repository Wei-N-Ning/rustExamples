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

// algorithms with rust L516
// declarative macros work on pattern and run code if that pattern
// matches; the previous example matches 0 - n expression (for 
// example, a number or a function that returns a number) and 
// inserts temp_vec.push(...) n times, iterating over the provided 
// expressions as a parameter
// #[macro_export]
// macro_rules! myvec {
//     ( $( $x:expr ),* ) => {
//         let mut temp_vec = Vec::new();
//         $( temp_vec.push($x) )*
//         temp_vec
//     };
// }

fn demo_myvec() {
}

// procedural macro
// #[derive(Debug, Clone, Copy)]
#[derive(Debug)]
struct Filename {
    name: String,
    ext: String,
}

fn demo_debug_filename() {
    let fname = Filename {
        name: String::from("there"),
        ext: String::from("cow"),
    };
    println!("{:?}", fname);
}

fn main() {
    demo_myvec();
    demo_debug_filename();
}
