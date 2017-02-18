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

use std::collections::HashSet;

fn is_pangram(s: &str) -> bool {

    let all_alphabets:HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let test_sentence:HashSet<char> = s.chars().collect();

    all_alphabets.is_subset(&test_sentence)
    

}


fn main() {

    assert_eq!(is_pangram("the quick brown fox jumps over the lazy dog"), true);
    assert_eq!(is_pangram("The quick brown fish jumps over the lazy dog"), false);
    assert_eq!(is_pangram("the five boxing wizards jump quickly"), true);
    assert_eq!(is_pangram("pack my box with five dozen liquor jugs"),true);

    println!("\nAll tests passed!!\n");
   
    //submit::submit(Question(10), Score(10));
}
