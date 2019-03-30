//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// recall map {} in perl and ruby
fn iterator_map() {
    let nums = vec![3, 1, 4, 1, 5, 9];
    let operand = 1;
    let plus_one = nums.iter().map(|x| x + operand);
    // map() is a closure; it can use the variables in the 
    // calling scope; same applies to filter()
    plus_one.for_each(|x| print!("{}, ", x));
    println!("");

    // the original vector is unmodified
    nums.iter().for_each(|elem| print!("{}, ", elem));
    println!("");
}

// recall grep {} in perl and ruby
fn iterator_filter() {
    let nums = vec![3, 1, 4, 1, 5, 9];
    let larger_then_three = nums.into_iter().filter(|&x| x > 3);
    larger_then_three.for_each(|elem| print!("{}, ", elem));
    println!("");
}

// consuming iterators

fn iterator_sum() {
    let nums = vec![3, 1, 4, 1, 5, 9];
    let sum:i32 = nums.iter().sum();
    println!("Sum: {}", sum);
}

fn iterator_max() {
    let nums = vec![3, 1, 4, 1, 5, 9];
    // max() returns an option instead of a real value
    let max:&i32 = nums.iter().max().unwrap();
    println!("Max: {}", max);

    let empty:Vec<i32> = [].to_vec();
    // if the container is empty, max() returns None
    println!("{:?}", empty.iter().max());
}

// the reduce part of map-reduce

fn iterator_fold() {
    let nums = vec![3, 1, 4, 1, 5, 9];
    let sum:i32 = nums.iter().fold(
        0, |sum, val| sum + val
    // init accu elem
    );
    println!("Reduce-sum: {}", sum);
}

// chaining

fn iterator_sqr_sum() {
    let nums = vec![3, 1, 4, 1, 5, 9];
    let sqr_sum:i32 = nums
        .iter()
        .map(|x| x * x)
        .sum();
    println!("Squared-sum: {}", sqr_sum);
}

// explicit evaluation

fn iterator_map_collect() {
    // map() is not evaluated unless something consumes its 
    // result; or calling .collect() will trigger evaluation
    let nums = vec![3, 1, 4, 1, 5, 9];
    let plus_one_iter = nums.iter().map(|elem| elem + 1);
    println!("Iter-map: {:?}", plus_one_iter);
    let modified:Vec<i32> = plus_one_iter.collect();
    println!("Modified: {:?}", modified);
}

fn iterator_infinite() {
    let nums:Vec<i32> = (1..)  // 1 to infinity
        .map(|elem| elem + 1)  // transform
        .filter(|elem| elem % 5 == 0)  // transform
        .take(7)  // take the first seven values computed
        .collect();  // trigger evaluation
    println!("Seven: {:?}", nums);
}
 
fn main() {
    iterator_map();
    iterator_filter();
    iterator_sum();
    iterator_max();
    iterator_fold();
    iterator_sqr_sum();
    iterator_map_collect();
    iterator_infinite();
}

