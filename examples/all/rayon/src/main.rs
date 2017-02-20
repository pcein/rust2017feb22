
extern crate rayon;

use rayon::prelude::*;

fn fib(n: u64) -> u64 {

    if n < 2 {
        n
    } else {
        fib(n - 1) + fib(n - 2)

    }
}

fn main() {
    let v:Vec<u64> = (0..47).collect();

    let v2:Vec<u64> = v.par_iter().map(|n| fib(*n)).collect();

    println!("{:?}", v2);
}
