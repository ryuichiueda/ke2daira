extern crate mecab;

use std::io;

fn read_line() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    line
}

fn main() {
    let line = read_line();
    let words: Vec<&str> = line
        .trim()
        .split(" ")
        .collect();

    println!("{:?}", words);
}
