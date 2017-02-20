
fn main() {
    let mut v1 = vec![1, 2, 3];

    let v2 = &v1; // borrowed (immutably)

    v2[0] = 10; // can't change
}


