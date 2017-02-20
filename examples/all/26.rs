
fn main() {

    let s1 = "abcd";

    let s2 = "pqrs";

    let r:Vec<(char, char)> = s1.chars().zip(s2.chars()).collect();

    println!("{:?}", r);

}
