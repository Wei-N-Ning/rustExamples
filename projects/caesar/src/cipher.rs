
// source: how to concat string (and how to  append char)
// https://stackoverflow.com/questions/30154541/how-do-i-concatenate-strings
pub fn cipher(cleartext: &String, key: u64) -> String {
    let mut enctext = String::new();
    for c in cleartext.chars() {
        enctext.push((c as u64 + key) as u8 as char);
    }
    enctext
}
