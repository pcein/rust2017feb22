
fn main() {
    let mut v1 = vec![1, 2, 3];

    let v2 = &mut v1; // borrowed (mutably)

    v2[0] = 10; 
}


