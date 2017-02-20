
/* Use of "any" and "all" */

fn main() {

    let v1 = vec![3, 5, 7, 4, 11, 13];

    let v2 = vec![3, 5, 7, 11, 13];

    let r1 = v1.iter().any(|x| x % 2 == 0);

    let r2 = v2.iter().all(|x| x % 2 == 1);

    println!("r1 = {}, r2 = {}", r1, r2);

}
    
