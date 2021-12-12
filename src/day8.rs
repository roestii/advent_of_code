#[derive(Debug)]
enum SimpleDigit {
    One(String),
    Four(String),
    Seven(String),
    Eight(String),
}

#[derive(Debug)]
enum ComplexDigit {
    Zero(String),
    Two(String),
    Three(String),
    Five(String),
    Six(String),
    Nine(String),
}

impl ComplexDigit {
    fn get_digits(input: Vec<&String> , one: &str, four: &str) -> Vec<Self> {
        let mut len_six = input.iter()
            .filter(|pattern| pattern.len() == 6)
            .fold(Vec::new(), |mut acc, pattern| {
                if !contains(pattern.to_string(), one.to_string()) {
                    acc.push(Self::Six(pattern.to_string()));
                    return acc;
                } else if !contains(pattern.to_string(), four.to_string()) {
                    acc.push(Self::Zero(pattern.to_string()));
                    return acc;
                }
                acc.push(Self::Nine(pattern.to_string()));
                acc
            });
        let six = len_six.iter().fold("", |mut six, pattern| {
            if let Self::Six(p) = pattern {
                six = p;
            }
            six
        });
        let len_five: Vec<&&String> = input.iter()
            .filter(|pattern| pattern.len() == 5)
            .collect();
        let three: Vec<&&&String> = len_five.iter().filter(|pattern| contains(pattern.to_string(), one.to_string())).collect();
        let three: String = three.first().unwrap().to_string();
        let others: Vec<&&&String> = len_five.iter().filter(|pattern| pattern.to_string() != three).collect(); 
        let (pattern_one, pattern_two) = (others[0], others[1]);
        if common_values(pattern_one.to_string(), six.to_string()) > common_values(pattern_two.to_string(), six.to_string()) {
            len_six.push(Self::Five(pattern_one.to_string()));
            len_six.push(Self::Two(pattern_two.to_string()));
        } else {
            len_six.push(Self::Five(pattern_two.to_string()));
            len_six.push(Self::Two(pattern_one.to_string()));
        }

        len_six.push(Self::Three(three));
        len_six
    }
}

impl SimpleDigit {
    fn get_digit(input: String) -> Option<Self> {
        match input.len() {
            2 => return Some(Self::One(input)),
            4 => return Some(Self::Four(input)),
            3 => return Some(Self::Seven(input)),
            7 => return Some(Self::Eight(input)),
            _ => None,
        }
    }
}

fn common_values(container: String, comp: String) -> u32 {
    let result = comp.chars()
        .fold(0, |mut res, c| {
            if container.contains(&c.to_string()) {
                res += 1;
            }
            res
        });
    result
}

fn contains(container: String, comp: String) -> bool {
    let result = comp.chars()
        .fold(true, |mut res, c| {
            if !container.contains(&c.to_string()) {
                res = false;
            }
            res
        });
    result
}

pub fn handle_input(input: Vec<&str>) {
    let values = input.iter()
        .fold(Vec::new(), |mut acc, line| {
            let (patterns, four_digit) = line.split_once("|").unwrap();
            let patterns: Vec<String> = patterns.split(" ")
                .filter(|pattern| !pattern.is_empty())
                .map(|pattern| {
                    let mut chars: Vec<char> = pattern.chars().collect();
                    chars.sort();
                    chars.into_iter().collect::<String>()
                })
                .collect();
            let four_digit: Vec<String> = four_digit.split(" ")
                .filter(|digit| !digit.is_empty())
                .map(|pattern| {
                    let mut chars: Vec<char> = pattern.chars().collect();
                    chars.sort();
                    chars.into_iter().collect::<String>()
                })
                .collect();
            acc.push((patterns, four_digit));
            acc
        });

    let numbers: Vec<u32> = values.iter()
        .fold(Vec::new(), |mut acc, (patterns, four_digit)| {
            let simple_digits: Vec<Option<SimpleDigit>> = patterns.iter()
                .map(|pattern| {
                    SimpleDigit::get_digit(pattern.to_string())
                })
                .filter(|pattern| {
                    match pattern {
                        Some(_) => true,
                        None => false,
                    }
                })
                .collect();
            let mut one = "";
            let mut four = "";

            for digit in &simple_digits {
                match digit {
                    Some(SimpleDigit::One(pattern)) => { one = &pattern; },
                    Some(SimpleDigit::Four(pattern)) => { four = &pattern; },
                    _ => {},
                }
            }
            let complex_patterns : Vec<&String> = patterns.iter()
                .filter(|pattern| SimpleDigit::get_digit(pattern.to_string()).is_none())
                .collect();
            let complex_digits = ComplexDigit::get_digits(complex_patterns, one, four);

            acc.push(calculate_number(four_digit.to_vec(), complex_digits));
            acc
        });
    println!("{}", numbers.iter().fold(0, |sum, number| sum + number));
}

fn calculate_number(four_digit: Vec<String>, complex_digits: Vec<ComplexDigit>) -> u32 {
    let number = four_digit.iter()
        .rev()
        .enumerate()
        .fold(0, |mut sum, (i, digit)| {
            if let Some(simple_digit) = SimpleDigit::get_digit(digit.to_string()) {
                return sum + match simple_digit {
                    SimpleDigit::One(_) => 1 * 10u32.pow(i as u32),
                    SimpleDigit::Four(_) => 4 * 10u32.pow(i as u32),
                    SimpleDigit::Seven(_) => 7 * 10u32.pow(i as u32),
                    SimpleDigit::Eight(_) => 8 * 10u32.pow(i as u32),
                }
            }

            for complex_digit in &complex_digits {
                match complex_digit {
                    ComplexDigit::Zero(pattern) => { if digit == pattern { sum += 0 } },
                    ComplexDigit::Two(pattern) => { if digit == pattern { sum += 2 * 10u32.pow(i as u32) } },
                    ComplexDigit::Three(pattern) => { if digit == pattern { sum += 3 * 10u32.pow(i as u32) } },
                    ComplexDigit::Five(pattern) => { if digit == pattern { sum += 5 * 10u32.pow(i as u32) } },
                    ComplexDigit::Six(pattern) => { if digit == pattern { sum += 6 * 10u32.pow(i as u32) } },
                    ComplexDigit::Nine(pattern) => { if digit == pattern { sum += 9 * 10u32.pow(i as u32) } },
                }
            }
            sum
        });
    number
}
