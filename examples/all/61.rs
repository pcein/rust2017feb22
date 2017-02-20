
fn main() {
    
    let v1 = vec![1,2,3];

    {
        let v2 = vec![10,20,30];
    }

    println!("{:?}", v1); 

    println!("{:?}", v2); // will not compile

}
