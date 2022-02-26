extern crate mecab;
use mecab::Tagger;

use std::io;
use std::env;

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

fn to_yomi(word: String) -> String {
    let tagger = Tagger::new(""); 
    let result: Vec<String> = tagger
        .parse_str(word)
        .split(",")
        .map(String::from)
        .collect();

    let mut ans = "".to_string();
    for (i, r) in result.iter().enumerate() {
        if i%10 == 7 {
            ans.push_str(r);
        }
    };

    ans
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut m_flag = false;
    for a in args {
        if a == "-m".to_string() {
            m_flag = true;
        };
    };

    let line = read_line();
    let words_org = tokenize(line);
    let mut words: Vec<String> = Vec::new();

    if m_flag {
        for w in words_org { 
            words.push(to_yomi( w.to_string()) ); 
        }
    }else{
        words = words_org;
    }

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
fn ke2daira_transform1() -> () {
    let words = tokenize("まつだいら けん".to_string());
    assert_eq!("けつだいら まん", transform(words));
}

#[test]
fn yomi_test1() -> () {
    let words = to_yomi("松平健は明日も元気。".to_string());
    assert_eq!("マツダイラケンハアシタモゲンキ。".to_string(), words);
}
