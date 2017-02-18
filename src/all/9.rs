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


fn hamming_distance(s1: &str, s2: &str) -> usize {

    s1.chars()
    .zip(s2.chars())
    .filter(|p| p.0 != p.1)
    .count()

}

fn main() {

    assert_eq!(hamming_distance("abc", "abc"), 0);
    assert_eq!(hamming_distance("abc", "def"), 3);
    assert_eq!(hamming_distance("abc", "adc"), 1);
    assert_eq!(hamming_distance("abc", "abf"), 1);
    assert_eq!(hamming_distance("abc", "amn"), 2);
    assert_eq!(hamming_distance("a", "b"), 1);

    println!("\nAll tests passed!!\n");
   
    //submit::submit(Question(9), Score(15));
}
