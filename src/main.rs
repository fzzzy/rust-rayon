
extern crate rayon;

use rayon::prelude::*;

fn process(i: &i32) {
    println!("Process: {:?}", i);
}

fn process_in_parallel() {
    let _: Vec<_> = (0i32..8i32)
        .collect::<Vec<i32>>()
        .par_iter()
        .map(|i| process(i))
        .collect();
}

fn main() {
    println!("Hello, world!");
    process_in_parallel();
}
