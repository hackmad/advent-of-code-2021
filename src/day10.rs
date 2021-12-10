use std::{collections::VecDeque, fs};

fn read(input_file: &str) -> Vec<String> {
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    contents.split("\n").map(String::from).collect()
}

fn closing_char(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("invalid character '{}'", c),
    }
}

fn corrupted_score(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("invalid character '{}'", c),
    }
}

fn autocomplete_score(s: &str) -> usize {
    s.chars().fold(0, |s, c| {
        let x = match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("invalid character '{}'", c),
        };
        s * 5 + x
    })
}

fn parse(input: &String) -> Result<String, char> {
    let mut stack = VecDeque::<char>::new();

    for c in input.chars() {
        match c {
            '(' => stack.push_back(c),
            '[' => stack.push_back(c),
            '{' => stack.push_back(c),
            '<' => stack.push_back(c),
            ')' | ']' | '}' | '>' => {
                let expected = stack.pop_back().map(closing_char);
                if let Some(e) = expected {
                    if c != e {
                        return Err(c); // Invalid closing.
                    }
                } else {
                    return Err(c); // Nothing on stack but closing.
                }
            }
            _ => panic!("invalid character '{}'", c),
        }
    }

    let autocomplete = stack
        .iter()
        .rev()
        .map(|&c| closing_char(c))
        .fold(String::new(), |s, c| format!("{}{}", s, c));
    Ok(autocomplete)
}

pub fn part_1_and_2(input_file: &str) {
    let lines = read(input_file);
    let (completed, errors): (Vec<_>, Vec<_>) = lines.iter().map(parse).partition(Result::is_ok);

    let error_score: usize = errors
        .iter()
        .map(|r| r.as_ref().unwrap_err())
        .map(|&c| corrupted_score(c))
        .sum();

    let mut completion_scores: Vec<usize> = completed
        .iter()
        .map(|r| r.as_ref().unwrap())
        .map(|s| autocomplete_score(&s))
        .collect();
    completion_scores.sort();
    let mid = completion_scores.len() / 2;

    println!("day 10: part 1 = {}", error_score);
    println!("day 10: part 2 = {}", completion_scores[mid]);
}
