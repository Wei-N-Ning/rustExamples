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

fn for_in_loop() {
    // for loop only takes iterator
    let l = [3, 1, 4, 1, 5, 9];

    for elem in l.into_iter() {
        print!("{}", elem);
    }

    // DO NOT
    // for elem in l {
    //     print!("{}", elem);
    // }
}

fn while_loop(num: i8) {
    let mut counter = num;
    while counter > 0 {
        print!("/");
        counter -= 1;
    }
}

fn vector_loop(nums: Vec<i32>) {
    // compiler creates an iterator out of the Vector
    println!("for-in-loop with vector");
    for elem in nums {
        print!("{}", elem);
    }

    // equivalent to 
    // let mut iter = IntoIterator::into_iter(nums);
    // loop {
    //     match iter.next() {
    //         Some(elem) => {
    //             print!("{}", elem);
    //         }
    //         None => { break }
    //     }
    // }

    // into_iter takes the ownership of the Vector (move the
    // elements to the loop)
    // in comparison:
    
    // for elem in nums.iter() { }
    // to iterate over an immutable reference
    // elem is &i32 !!!

    // for elem in nums.iter_mut() { }
    // to iterate over an mutable ref

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
    for_in_loop();
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
