extern crate mecab;
use mecab::Tagger;

use std::io;
use std::env;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn check_help_flag(args: &Vec<String>) -> () {
    if args.len() > 1 {
        if args[1] == "-h" {
            eprintln!("ke2daira {}", VERSION);
            eprintln!("Copyright (C) 2022 Ryuichi ueda\n");
            eprintln!("usage: ke2daira [-m] [f1.n] [f2.n]\n");
            eprintln!("Released under the MIT license");
            eprintln!("https://github.com/ryuichiueda/ke2daira");
            std::process::exit(1);
        }
    }
}

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

fn slice_pos(word: &String, pos: usize) -> usize {
    match word.char_indices().nth(pos) {
        Some(x) => Some(x).unwrap().0, 
        _       => word.len(),
    }

}


fn transform(mut words: Vec<String>,
            (w1_pos, w2_pos, w1_len, w2_len): (usize, usize, usize, usize) 
    ) -> String {
    let w1 = &words[w1_pos].clone();
    let w2 = &words[w2_pos].clone();

    let w1_spos = slice_pos(&w1, w1_len);
    let w2_spos = slice_pos(&w2, w2_len);

    words[w1_pos] = format!("{}{}", &w2[..w2_spos], &w1[w1_spos..]);
    words[w2_pos] = format!("{}{}", &w1[..w1_spos], &w2[w2_spos..]);

    words.join(" ")
}


fn to_yomi(word: String) -> String {
    let tagger = Tagger::new(""); 
    let result_lines: Vec<String> = tagger
        .parse_str(word.clone())
        .split("\n")
        .map(String::from)
        .collect();

    let mut ans = "".to_string();

    for line in result_lines {
        let tokens: Vec<String> = line.split(",")
                     .map(String::from)
                     .collect();

        if tokens.len() > 7 {
            ans.push_str(&tokens[7]);
        }else if tokens.len() > 1 {
            ans.push_str(&word);
        };
    }
    ans
}

fn use_mecab(args: &Vec<String>) -> bool {
    for a in args {
        if *a == "-m".to_string() {
            return true;
        };
    };
    false
}

fn solve_length(arg: String) -> usize {
    let delim = match arg.chars().nth(1) {
        Some(x) => x,
        None    => return 1,
    };

    if delim != '.' {
        panic!("Argument error");
    };

    match arg.chars().nth(2) {
        Some(x) => x as usize - 48,
        None    => panic!("No word length"),
    }
}

fn set_positions(args: &Vec<String>) -> (usize, usize, usize, usize) {
    let mut first_changed = false;
    let mut first = 0;
    let mut first_length = 1;

    for a in args {
        let head = match a.chars().nth(0) {
            Some(x) => x,
            None    => '0',
        };

        if head >= '1' && head <= '9' {
            if first_changed {
                let second = head as usize - 49;
                let second_length = solve_length(a.to_string());
                return (first, second, first_length, second_length);
            }else{
                first = head as usize - 49;
                first_length = solve_length(a.to_string());
                first_changed = true;
            }
        };
    };
    (0, 1, 1, 1)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    check_help_flag(&args);
    let mecab_flag = use_mecab(&args);
    let fields = set_positions(&args);

    let line = read_line();
    let words: Vec<_> = tokenize(line)
        .iter()
        .map(String::to_string)
        .map(|w| if mecab_flag{to_yomi(w)}else{w})
        .collect();

    match words.len() {
        0 => (),
        1 => println!("{}", words[0]),
        _ => println!("{}", transform(words, fields)),
    };
}

/////////////////////////////////////
//TEST PART
/////////////////////////////////////
#[test]
fn ke2daira_transform1() -> () {
    let words = tokenize("まつだいら けん".to_string());
    assert_eq!("けつだいら まん", transform(words, (0, 1, 1, 1) ));
}

#[test]
fn ke2daira_transform2() -> () {
    let words = tokenize("まつだいら けん さんじょう".to_string());
    assert_eq!("まつだいら さん けんじょう", transform(words, (1, 2, 1, 1) ));
}

#[test]
fn ke2daira_transform3() -> () {
    let words = tokenize("まつだいら けん".to_string());
    assert_eq!("まつだいら けん", transform(words, (0, 1, 0, 0) ));
}

#[test]
fn ke2daira_transform4() -> () {
    let words = tokenize("まつだいら けん".to_string());
    assert_eq!("けんだいら まつ", transform(words, (0, 1, 2, 2) ));

    let words = tokenize("まつだいら けんさん".to_string());
    assert_eq!("けだいら まつんさん", transform(words, (0, 1, 2, 1) ));
}

#[test]
fn yomi_test1() -> () {
    let words = to_yomi("松平さんは明日も元気。".to_string());
    assert_eq!("マツダイラサンハアシタモゲンキ。".to_string(), words);
}

#[test]
fn yomi_test2() -> () {
    let words = to_yomi("ゲバラ".to_string());
    assert_eq!("ゲバラ".to_string(), words);
}
