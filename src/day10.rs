use std::collections::HashMap;

enum Delimiter {
    Opening(char),
    Closing(char),
    Nothing
}

impl From<char> for Delimiter {
    fn from(delimiter: char) -> Self {
        match delimiter {
            '(' => Self::Opening('('),
            '{' => Self::Opening('{'),
            '[' => Self::Opening('['),
            '<' => Self::Opening('<'),
            ')' => Self::Closing('('),
            '}' => Self::Closing('{'),
            ']' => Self::Closing('['),
            '>' => Self::Closing('<'),
            _ => Self::Nothing,
        }
    }
}

pub fn handle_input(input: Vec<&str>) {
    let mut errs: HashMap<char, u32> = HashMap::new();
    let arr =  [
        ('(', 3),
        ('[', 57),
        ('{', 1197),
        ('<', 25137),
    ];
    arr.iter().for_each(|(c, v)| {
        errs.insert(*c, *v);
    });

    let mut attachments: HashMap<char, u32> = HashMap::new();
    let arr = [
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4),
    ];
    arr.iter().for_each(|(c, v)| {
        attachments.insert(*c, *v);
    });

    let sum = input.iter()
        .fold(0, |mut sum, line| {
            let mut stack = Vec::new();
            line.chars().for_each(|c| {
                match c.into() {
                    Delimiter::Opening(del) => stack.push(Delimiter::Opening(del)),
                    Delimiter::Closing(del) => {
                        match stack.pop().unwrap() {
                            Delimiter::Opening(d) => {
                                if d != del {
                                    sum += *errs.get(&del).unwrap();
                                }
                            }
                            _ => {},
                        }
                    },
                    _ => {},
                }
            });
            sum
        });

    let mut incomplete: Vec<usize> = input.iter()
        .filter(|line| !is_corrupted(line))
        .fold(Vec::new(), |mut acc, line| {
            let mut stack = Vec::new();
            line.chars().for_each(|c| {
                match c.into() {
                    Delimiter::Opening(del) => stack.push(del),
                    Delimiter::Closing(del) => { stack.pop(); },
                    _ => {},
                }
            });
            acc.push(stack.iter().rev().fold(0, |sum, del| {
                sum * 5 + *attachments.get(del).unwrap() as usize
            }));
            acc
        });
    incomplete.sort();
    println!("{}", incomplete[(incomplete.len()-1) / 2]);
}

fn is_corrupted(line: &str) -> bool {
    let mut stack = Vec::new();
    let mut res = false;
    line.chars().for_each(|c| {
        match c.into() {
            Delimiter::Opening(del) => stack.push(Delimiter::Opening(del)),
            Delimiter::Closing(del) => {
                match stack.pop().unwrap() {
                    Delimiter::Opening(d) => {
                        if d != del {
                            res = true;
                        }
                    }
                    _ => {},
                }
            },
            _ => {},
        }
    });
    res
}
