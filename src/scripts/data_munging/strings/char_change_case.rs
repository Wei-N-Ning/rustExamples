//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// note to_ascii_uppercase() does not need 'use std::ascii::AsciiExt'
// this module is deprecated

fn demo_char_change_case(s: &String) {
    let mut result = String::new();
    s.chars().for_each(|c| result.push(c.to_ascii_uppercase()));
    println!("{}", result);
}

fn main() {
    demo_char_change_case(&"there is acow".to_string());
}