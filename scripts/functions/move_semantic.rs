//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

fn consumer(nums: Vec<i32>) {
    // value moved into this function
    // the ownership of nums is transferred to this function
    for elem in nums {
        print!("{}", elem);
    }
    println!("");
}

fn reader(nums: &Vec<i32>) {
    // use reference instead
    for elem in nums {
        
        // can not do that because nums is an immutable ref
        // *elem += 1;

        print!("{}", *elem);
    }
    println!("");
}

fn modifier(nums: &mut Vec<i32>) {
    // use mutable reference (note the syntax here and on the 
    // caller side)
    for mref in nums {
        // not a problem
        *mref *= 10;

        // don't have to de-ref here; rust compiler does it
        print!("{}", mref);
    }
    println!("");
}

fn main() {
    let nums = vec![3, 1, 4, 1, 5, 9];
    consumer(nums);
    // vector nums is moved; its ownership is lost
    // using this vector after move is illegal
    // println!("{:?}", nums);

    let mut vals = vec![3, 1, 4, 1, 5, 9];
    reader(&vals);
    // ownership to vals remains
    println!("{:?}", vals);

    // native types, i, str etc... implement the copy-trait
    // therefore this problem does not happen

    modifier(&mut vals);
    // ownership to vals remains (changing vals to mut)
    println!("{:?}", vals);
}