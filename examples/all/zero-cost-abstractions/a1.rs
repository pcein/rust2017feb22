
const N: u64 = 6;

fn main() {
    let r = (0..N).fold(0, |sum, i| sum+i);
    println!("{}", r);
}
