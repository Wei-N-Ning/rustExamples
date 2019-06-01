
// see: Rayon- Data Parallelism for Fun and Profit — Nicholas Matsakis
use rayon::prelude::*;

fn sum_of_squares() {
    // range to vec, see
    // https://stackoverflow.com/questions/26033976/how-do-i-create-a-vec-from-a-range-and-shuffle-it
    let v: Vec<i64> = (1..1231).collect();
    let result: i64 = v.par_iter()
         .map(|&i| i * i)
         .sum();
    println!("sum of sqr: {}", result);
}

pub fn demo() {
    sum_of_squares();
}