//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

fn loop_loop(num: i8) {
    let mut counter = num;
    loop {
        if counter <= 0 {
            return;
        }
        print!(".");
        counter -= 1;
    }
}

fn for_loop(num: i8) {
    // .. creates a range object (for in is a syntactic sugar)
    // for supports iterator 
    for _n in 0..num {
        print!("-");
    }
}

fn while_loop(num: i8) {
    let mut counter = num;
    while counter > 0 {
        print!("/");
        counter -= 1;
    }
}

fn vector_loop(nums: Vec<i32>) {
    for elem in nums {
        println!("{}", elem);
    }
}

fn continue_break(nums: Vec<i32>) {
    for elem in nums {
        if elem / 2 == 0 {
            continue;
        }
        println!(".. {}", elem);
        break;
    }
}

fn main() {
    loop_loop(3);
    for_loop(3);
    while_loop(3);

    // prefer the micro 
    // let mut nums = Vec::new();
    // nums.push(1);
    // nums.push(2);
    let nums = vec![1, 2];
    vector_loop(nums);
    
    // can NOT be vec![3..4] or vec!3..4
    let nums = vec![3, 4];
    continue_break(nums);

    println!("+ done");
}
