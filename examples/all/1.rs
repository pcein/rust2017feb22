
fn main() {    
    let v1 = vec![3, 5, 11, 13, 2, 17, 6];
    let v2 = vec![3, 5, 11, 13,  17];
    
    let r1 = v1.iter().any(|x| x%2 == 0);
    let r2 = v2.iter().any(|x| x%2 == 0);

    println!("r1 = {}", r1);
    println!("r2 = {}", r2);
}
    
