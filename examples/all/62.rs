
fn main() {
    
    let p: &Vec<i32>;

    {
        let v = vec![1, 2, 3];
        p = &v;
    }
    // will not compile
    // discuss "lifetimes", the third central
    // Rust concept.
   
    println!("{}", p[0]);
}
