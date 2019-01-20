// rust version:
// Cargo.toml says 2018 -> rust major version 2018

// source: rs in 7d

fn compute(i_operand: i32) -> i32 {
    // return value:
    // return i_operand * 10
    // or
    // i_operand * 10
    // or
    let _res = i_operand;
    // _res * 10;

    // casting operator: "as"
    // truncating high bits
    (_res as i8) as i32 * 200
}

fn mutable_value() -> i32 {
    // without mut keyword, each modification must start with
    // a new variable declaration: let var = var * 20;
    let mut var = 10;
    var *= 20;
    var /= 20;
    var as i32
}

fn main() {
    // prefix's value is computed at compile time not run time
    // therefore it can not take function return value
    let prefix = format!("{}", ": ");
    println!(
        "{}{}, {}", 
        prefix, 
        compute(0b100000001),
        mutable_value()
    );
}
