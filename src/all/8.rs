/* These lines are required to facilitate
 * the score submission process. Don't worry
 * about them!
 */
#![allow(dead_code, unused_imports)]

mod submit;
mod myconfig;

extern crate hyper;

use submit::{Question, Score};

/*--------------------------------------------*/

// sum of those numbers from 2 to n-1 which are
// multiples of any number in v
// example: n = 10 and v = vec![3,5]
// sum = 3 + 5 + 6 + 9 = 23
fn sum_multiples_till(n: i32, v:Vec<i32>) -> i32 {

    /* add a line here */

    /* Hint: you need to FILTER out all numbers
     * from 1 .. n which are divisible by ANY
     * number in v.
     * You need these 3 functions: filter, any and sum
     */

}

fn main() {

    assert_eq!(sum_multiples_till(10, vec![3,5]), 23);
    assert_eq!(sum_multiples_till(4, vec![3,5]), 3);
    assert_eq!(sum_multiples_till(6, vec![3,5]), 8);
    assert_eq!(sum_multiples_till(20, vec![7, 13, 17]), 51);
    assert_eq!(sum_multiples_till(150, vec![5, 6, 8]), 4419);
 
    println!("\nAll tests passed!!\n");
   
    //submit::submit(Question(8), Score(15));
}
