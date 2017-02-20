
/* if as an "expression" */

fn foo(x: i32, y: i32) -> i32 {
    if x == y {
        10
    } else {
        20
    }
}

fn main() {

    println!("foo(1,1) = {}", foo(1,1));
    println!("foo(1,2) = {}", foo(1,2));
}
