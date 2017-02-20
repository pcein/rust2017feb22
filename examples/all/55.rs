
fn sum_elements(v: &Vec<i32>) -> i32 {
    
    v[0] + v[1] + v[2]

}

fn multiply_elements(v: &Vec<i32>) -> i32 {


    v[0] * v[1] * v[2]

}

fn main() {
    
    let v = vec![2, 3, 4];

    let r1 = sum_elements(&v);


    let r2 = multiply_elements(&v);

    println!("r1 = {}, r2 = {}", r1, r2);

}
