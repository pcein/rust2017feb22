
fn main() {
    let v1 = vec![1, 2, 3];

    let v2 = &v1; // borrowed!

    let v3 = &v1; // once again!
}


