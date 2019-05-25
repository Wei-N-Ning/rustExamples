//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

fn platform_agnostic_code() {
    if cfg!(target_os = "macos") {
        println!("Mac OS foo");
    }
    else if cfg!(target_os = "linux") {
        println!("Linux foo");
    }
    else {
        println!("Other foo");
    }
}

#[cfg(target_os = "linux")]
#[derive(Debug)]
struct LinuxClass {}

fn main() {
    platform_agnostic_code();
}