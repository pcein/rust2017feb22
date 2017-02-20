

fn main() {
    let s = "abc def ijk";

    for word in s.split_whitespace() {
        println!("{}", word);
    }
}
