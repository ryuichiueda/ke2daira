extern crate mecab;

use std::io;

fn read_line() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    line
}

fn tokenize(line: String) -> Vec<String> {
    line.trim()
        .split(" ")
        .map(String::from)
        .collect()
}

fn transform(words: Vec<String>) -> String {
    let head1 = match words[0].chars().nth(0) {
        Some(x) => x,
        None    => panic!("Empty string"),
    };

    let head2 = match words[1].chars().nth(0) {
        Some(x) => x,
        None    => panic!("Empty string"),
    };

    let tail1 = words[0].chars().skip(1).collect::<String>();
    let tail2 = words[1].chars().skip(1).collect::<String>();

    let mut ans = format!("{}{} {}{}", head2, tail1, head1, tail2);
    for w in &words[2..] {
        ans.push_str(" ");
        ans.push_str(w);
    };
    ans
}

fn main() {
    let line = read_line();
    let words = tokenize(line);

    match words.len() {
        0 => (),
        1 => println!("{}", words[0]),
        _ => println!("{}", transform(words)),
    };
}

/////////////////////////////////////
//TEST PART
/////////////////////////////////////
#[test]
fn test1() -> () {
    let words = tokenize("まつだいら けん".to_string());
    assert_eq!("けつだいら まん", transform(words));
}
