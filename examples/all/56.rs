
fn main() {
    let v1 = vec![1, 2, 3];

    let v2 = v1; // moved!

    println!("{:?}", v1); // will not compile
}


