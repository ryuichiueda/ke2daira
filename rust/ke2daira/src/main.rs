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

fn slice_pos(word: &String, pos: usize) -> usize {
    match word.char_indices().nth(pos) {
        Some(x) => Some(x).unwrap().0, 
        _       => word.len(),
    }

}

fn transform(words: Vec<String>,
            (w1_pos, w2_pos, w1_len, w2_len): (usize, usize, usize, usize) 
    ) -> String {
    let w1_spos = slice_pos(&words[w1_pos], w1_len);
    let head1 = &words[w1_pos][..w1_spos];
    let tail1 = &words[w1_pos][w1_spos..];

    let w2_spos = slice_pos(&words[w2_pos], w2_len);
    let head2 = &words[w2_pos][..w2_spos];
    let tail2 = &words[w2_pos][w2_spos..];

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

fn _to_yomi(word: String, dic_length: usize) -> String {
    println!("{:?}", word);
    let tagger = Tagger::new(""); 
    let result: Vec<String> = tagger
        .parse_str(word)
        .split(",")
        .map(String::from)
        .collect();

    println!("{:?}", result);

    let mut ans = "".to_string();
    let mut org = "";
    for (i, r) in result.iter().enumerate() {
        if i%dic_length == 0 {
            org = r; 
        };

        if i%dic_length == 7 {
            if r == "*"{
                ans.push_str(org);
            }else{
                ans.push_str(r);
            }
        }
    };

    ans
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

    let mecab_flag = use_mecab(&args);
    let fields = set_positions(&args);

    let line = read_line();
    let raw_words = tokenize(line);

    let words = if mecab_flag {
        let mut yomi_words: Vec<String> = Vec::new();
        for w in raw_words { 
            yomi_words.push(to_yomi( w.to_string())); 
        }
        yomi_words
    }else{
        raw_words
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
