/* These lines are required to facilitate
 * the score submission process. Don't worry
 * about them!
 */
#![allow(dead_code, unused_imports, unused_variables)]

mod submit;
mod myconfig;

extern crate hyper;

use submit::{Question, Score};

/*--------------------------------------------*/

// A simple Finite State Machine model of an elevator
// Two floors (Ground, First) and a Button (Up, Down).
// (Ground -> Up => First), (First -> Down => Ground)
// (Ground -> Down => Ground), (First -> Up => First)

#[derive(Debug, PartialEq)]
enum Floor {
    Ground,
    First,
}

#[derive(Debug, PartialEq)]
enum Button {
    Up,
    Down,
}

use Floor::*;
use Button::*;

fn next_floor(curr_floor: Floor, btn: Button) -> Floor {
    match (curr_floor, btn) {
        (Ground, Up)   => First,
        (Ground, Down) => Ground,
        (First, Up)    => First,
        (First, Down)  => Ground,
    }
}

fn main() {

    assert_eq!(next_floor(Ground, Up), First);
    assert_eq!(next_floor(Ground, Down), Ground);
    assert_eq!(next_floor(First, Up), First);
    assert_eq!(next_floor(First, Down), Ground);

    println!("\nAll tests passed!!\n");
   
    //submit::submit(Question(14), Score(10));
}
