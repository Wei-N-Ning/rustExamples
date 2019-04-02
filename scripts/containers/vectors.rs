//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

fn creation() {
    // specify type of element
    let mut elements: Vec<i32> = Vec::new(); elements.push(1);
    println!("{:?}", elements);

    // can not infer type for T
    // let mut a = Vec::new();  

    // turbofish 
    let elems = Vec::<i32>::new();
    println!("{:?}", elems);

    // to create a vec of ten 1s!
    assert_eq!(vec![1; 10], vec![1; 10])
}

fn main() {
    creation();
}
