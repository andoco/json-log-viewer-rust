use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut i = 0;
    for line in stdin.lock().lines() {
        println!("{}: {}", i, line.unwrap());
        i += 1;
    }
}
