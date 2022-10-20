use std::io;
#[warn(unused_variables)]
fn main() {
    let input = "adbcdaDd";
    let mut count = 0;
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    for c in input.chars() {
        if c == s.chars().next().unwrap() {
            count += 1;
        }
    }
    println!("{}: {}", input, count);
}
