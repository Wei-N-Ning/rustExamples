//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

fn demo_assertion() {
    // assertion
    assert_eq!(1, 1);
    assert_eq!(vec![1; 5], vec![1; 5]);  // repeat 5 times
}

fn demo_env_var() {
    // environment macro
    // both are evaluated at //// compile time ////

    // env! will cause compilation error if not defined
    assert!(env!("USER") != "doom guy");
    
    // option_env! returns an option object
    let var: Option<&'static str> = option_env!("VAR");
    // this environment variable is not defined, hence None
    // run this script with bash:
    // VAR=1337 ./common_macros.rs 
    // Some("1337")
    println!("{:?}", var);
}

fn demo_print_and_format() {
    // (e)print! std(err)out, no newline
    // (e)println! std(err)out, newline 
    
    // print..! supports positional placeholders
    println!("{0} {1} {0} {1}", 'a', 'b');
    // named placeholders
    println!("{name} {type}", name="cow", type="thereis");

    // format!
    assert_eq!("doom", format!("{1}{0}", "om", "do"));

    // concat!
    assert_eq!("doom", concat!("d", "oo", "m"));
}

fn demo_debugging() {
    println!("{}, line {}, column {}, module path {}",
        file!(),
        line!(),
        column!(),
        module_path!()
    );
}

fn demo_include() {
    // include!() is a dangerous macro!!!
    // include!() evaluates the given text file as rust source
    // code then stores the output in the variable;

    let file_o = include_str!("macro_rules.rs");
    println!("{}", file_o);

    // let byte_array: &'static [u8; N] = 
    //      include_bytes!("some_binary_file");
}

fn main() {
    demo_assertion();
    demo_env_var();
    demo_print_and_format();
    demo_debugging();
    demo_include();
}

