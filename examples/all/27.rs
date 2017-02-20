
fn main() {

    let v1 = vec![1,2,3];

    let v2 = vec![4,5,6];

    let r:Vec<i32> = v1.iter().zip(v2.iter()).map(|p| p.0 + p.1).collect();

    println!("{:?}", r);

}
