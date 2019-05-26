use std::env;
mod cipher;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 => {
            let cleartext = &args[1];
            let key = &args[2];
            // parse the _number
            let number: u64 = match key.parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    eprintln!("error: second argument not an integer");
                    return;
                },
            };
            let enctext = cipher::cipher(&cleartext, number);
            println!("{}", enctext);
        },
        // all the other cases
        _ => {
            println!("usage: caesar <cleartext> <key>");
        }
    }
}
