//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

use std::env;

fn display_raw_arguments() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}

fn parse_arguments_from_scratch() {
    // see
    // https://doc.rust-lang.org/rust-by-example/std_misc/arg/matching.html

    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 => {
            let cleartext = &args[1];
            let key = &args[2];
            // parse the _number
            let _number: i32 = match key.parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    eprintln!("error: second argument not an integer");
                    return;
                },
            };
            println!("{} / {}", cleartext, key);
        },
        // all the other cases
        _ => {
            println!("usage: caesar <cleartext> <key>");
        }
    }
}

fn main() {
    display_raw_arguments();
    parse_arguments_from_scratch();
}
