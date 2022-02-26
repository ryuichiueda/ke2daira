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

fn transform(words: Vec<String>, w1_pos: usize, w2_pos: usize) -> String {
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut m_flag = false;

    let mut first_changed = false;
    let mut first = 0;
    let mut second = 1;

    for a in args {
        if a == "-m".to_string() {
            m_flag = true;
            continue;
        };

        let head = match a.chars().nth(0) {
            Some(x) => x,
            None    => panic!("Empty argument"),
        };

        if head >= '1' && head <= '9' {
            if first_changed {
                second = head as usize - 49;
            }else{
                first = head as usize - 49;
                first_changed = true;
            }
        };
    };

    let line = read_line();
    let words_org = tokenize(line);
    let mut words: Vec<String> = Vec::new();

    if m_flag {
        let width = solve_mecab_dict_width();
        for w in words_org { 
            words.push(to_yomi( w.to_string(), width)); 
        }
    }else{
        words = words_org;
    }

    match words.len() {
        0 => (),
        1 => println!("{}", words[0]),
        _ => println!("{}", transform(words, first, second)),
    };
}

/////////////////////////////////////
//TEST PART
/////////////////////////////////////
#[test]
fn ke2daira_transform1() -> () {
    let words = tokenize("まつだいら けん".to_string());
    assert_eq!("けつだいら まん", transform(words, 0, 1));
}

#[test]
fn ke2daira_transform2() -> () {
    let words = tokenize("まつだいら けん さんじょう".to_string());
    assert_eq!("まつだいら さん けんじょう", transform(words, 1, 2));
}

#[test]
fn yomi_test1() -> () {
    let w = solve_mecab_dict_width();
    let words = to_yomi("松平さんは明日も元気。".to_string(), w);
    assert_eq!("マツダイラサンハアシタモゲンキ。".to_string(), words);
}
