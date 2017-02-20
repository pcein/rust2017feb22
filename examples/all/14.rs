
/* if as an "expression" */

fn main() {
    let x = 1;
    let y = 1;

    let r = if x == y { 10 } else { 20 };

    println!("r = {}", r);
}
