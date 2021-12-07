use std::cmp::Ordering;

#[derive(Eq, Debug)]
struct MatchingLine {
    matching: (u32, String), 
}

impl Ord for MatchingLine {
    fn cmp(&self, other: &Self) -> Ordering {
        self.matching.0.cmp(&other.matching.0)
    }
}

impl PartialOrd for MatchingLine {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for MatchingLine {
    fn eq(&self, other: &Self) -> bool {
        self.matching.0 == other.matching.0
    }
}

pub fn handle_input(input: Vec<&str>) -> Option<()> {
    let inverted_input = invert_input(&input)?;

    let gamma = get_most_common(&inverted_input);
    let epsilon: Vec<u32> = gamma.iter().map(|x| {
        if *x == 1 { 0 } else { 1 }
    }).collect();
    
    let gamma = to_dezimal(gamma);
    let epsilon = to_dezimal(epsilon);
    let result = gamma * epsilon; 
    
    println!("gamma: {}, epsilon: {} => {}", gamma, epsilon, result);

    let oxygen_generator = get_oxygen_generator(&input);
    let co2_generator = get_co2_generator(&input);

    println!("o2: {}, co2: {} => {}", oxygen_generator, co2_generator, oxygen_generator * co2_generator);
    
    Some(())
}

fn get_co2_generator(input: &Vec<&str>) -> u32 {
    let mut co2_generator = input.clone();

    let initial: Vec<u32> = co2_generator.first().unwrap().chars().fold(Vec::new(), |mut acc, _| {
        acc.push(0);
        acc
    });

    for bit in 0..initial.len() {
        if co2_generator.len() == 1 {
            break;
        }

        let sums = co2_generator.iter().fold(initial.clone(), |mut acc, line| {
            line.chars().map(|c| c.to_digit(2).unwrap()).zip(&mut acc).for_each(|(n, sum)| {
                *sum += n;
            });
            acc
        });

        let most_common: Vec<u32> = sums.iter()
            .map(|sum| { if *sum as f32 >= (co2_generator.len() as f32 / 2_f32) { 0 } else { 1 } })
            .collect();

        let remove_idx = co2_generator.iter().enumerate().fold(Vec::new(), |mut acc, (idx, line)| {
            line.chars().enumerate().for_each(|(i, c)| {
                if i == bit && c.to_digit(2).unwrap() != most_common[bit]  {
                    acc.push(idx);
                }
            });
            acc
        });

        co2_generator = co2_generator.iter().enumerate().filter(|(idx, element)| {
            !remove_idx.contains(idx)    
        }).map(|(idx, element)| *element).collect();

    }

    let dezimal = to_dezimal(co2_generator.first().unwrap().chars().map(|c| c.to_digit(10).unwrap()).collect());
    dezimal
}

fn get_oxygen_generator(input: &Vec<&str>) -> u32 {
    let mut oxygen_generator = input.clone();

    let initial: Vec<u32> = oxygen_generator.first().unwrap().chars().fold(Vec::new(), |mut acc, _| {
        acc.push(0);
        acc
    });

    for bit in 0..initial.len() {
        if oxygen_generator.len() == 1 {
            break;
        }

        let sums = oxygen_generator.iter().fold(initial.clone(), |mut acc, line| {
            line.chars().map(|c| c.to_digit(2).unwrap()).zip(&mut acc).for_each(|(n, sum)| {
                *sum += n;
            });
            acc
        });

        let most_common: Vec<u32> = sums.iter()
            .map(|sum| { if *sum as f32 >= (oxygen_generator.len() as f32 / 2_f32) { 1 } else { 0 } })
            .collect();

        let remove_idx = oxygen_generator.iter().enumerate().fold(Vec::new(), |mut acc, (idx, line)| {
            line.chars().enumerate().for_each(|(i, c)| {
                if i == bit && c.to_digit(2).unwrap() != most_common[bit]  {
                    acc.push(idx);
                }
            });
            acc
        });

        oxygen_generator = oxygen_generator.iter().enumerate().filter(|(idx, element)| {
            !remove_idx.contains(idx)    
        }).map(|(idx, element)| *element).collect();

    }

    let dezimal = to_dezimal(oxygen_generator.first().unwrap().chars().map(|c| c.to_digit(10).unwrap()).collect());
    dezimal
}

fn to_dezimal(seq: Vec<u32>) -> u32 {
    let mut sum = 0;
    for (i, bin) in seq.iter().rev().enumerate() {
        sum += bin * (2 as u32).pow(i as u32);
    }
    sum
}

fn get_most_common(input: &Vec<Vec<u32>>) -> Vec<u32> {
    let most_common = input.iter().fold(Vec::new(), |mut acc, line| {
        let count_ones = line.iter().filter(|x| **x == 1).count();

        if count_ones > line.len() / 2 {
            acc.push(1);
        } else {
            acc.push(0);
        }
        acc
    });

    most_common
}

fn invert_input(input: &Vec<&str>) -> Option<Vec<Vec<u32>>> {
    let initial: Vec<Vec<u32>> = input.first()?.chars().fold(Vec::new(), |mut acc, _| {
        acc.push(Vec::new());
        acc
    }); 
    let inverted = input.iter().fold(initial, |mut acc, line| {
        line.chars().zip(&mut acc).for_each(|(c, vec)| {
            vec.push(c.to_digit(2).unwrap());
        });
        acc
    }); 
    
    Some(inverted)
}
