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

fn transform(words: Vec<String>, (w1_pos, w2_pos): (usize, usize) ) -> String {
    let head1 = match words[w1_pos].chars().nth(0) {
        Some(x) => x,
        None    => panic!("Empty string"),
    };

    let head2 = match words[w2_pos].chars().nth(0) {
        Some(x) => x,
        None    => panic!("Empty string"),
    };

    let tail1 = words[w1_pos].chars().skip(1).collect::<String>();
    let tail2 = words[w2_pos].chars().skip(1).collect::<String>();

    let w1 = format!("{}{}", head2, tail1);
    let w2 = format!("{}{}", head1, tail2);

    let mut ans: String = "".to_string();
    for (i, w) in words.iter().enumerate() {
        if i == w1_pos {
            ans.push_str(&w1);
        }else if i == w2_pos {
            ans.push_str(&w2);
        }else{
            ans.push_str(w);
        }

        if i != words.len() - 1 {
            ans.push_str(" ");
        }
    }

    ans
}

fn solve_mecab_dict_width() -> usize {
    let tagger = Tagger::new(""); 
    let result: Vec<String> = tagger
        .parse_str("あ")
        .split(",")
        .map(String::from)
        .collect();

    result.len() - 1
}

fn to_yomi(word: String, dic_length: usize) -> String {
    let tagger = Tagger::new(""); 
    let result: Vec<String> = tagger
        .parse_str(word)
        .split(",")
        .map(String::from)
        .collect();

    let mut ans = "".to_string();
    for (i, r) in result.iter().enumerate() {
        if i%dic_length == 7 {
            ans.push_str(r);
        }
    };

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

fn set_positions(args: &Vec<String>) -> (usize, usize) {
    let mut first_changed = false;
    let mut first = 0;

    for a in args {
        let head = match a.chars().nth(0) {
            Some(x) => x,
            None    => '0',
        };

        if head >= '1' && head <= '9' {
            if first_changed {
                let second = head as usize - 49;
                return (first, second);
            }else{
                first = head as usize - 49;
                first_changed = true;
            }
        };
    };
    (0, 1)
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let mecab_flag = use_mecab(&args);
    let fields = set_positions(&args);

    let line = read_line();

    let words = if mecab_flag {
        let mut yomi_words: Vec<String> = Vec::new();
        let width = solve_mecab_dict_width();
        for w in tokenize(line) { 
            yomi_words.push(to_yomi( w.to_string(), width)); 
        }
        yomi_words
    }else{
        tokenize(line)
    };

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
    assert_eq!("けつだいら まん", transform(words, (0, 1) ));
}

#[test]
fn ke2daira_transform2() -> () {
    let words = tokenize("まつだいら けん さんじょう".to_string());
    assert_eq!("まつだいら さん けんじょう", transform(words, (1, 2) ));
}

#[test]
fn yomi_test1() -> () {
    let w = solve_mecab_dict_width();
    let words = to_yomi("松平さんは明日も元気。".to_string(), w);
    assert_eq!("マツダイラサンハアシタモゲンキ。".to_string(), words);
}
