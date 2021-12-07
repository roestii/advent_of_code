pub fn handle_input(input: Vec<&str>) -> Option<()> {
    let input = invert_input(&input)?;
    let gamma = get_most_common(input);
    let epsilon: Vec<u32> = gamma.iter().map(|x| {
        if *x == 1 { 0 } else { 1 }
    }).collect();
    
    let gamma = to_dezimal(gamma);
    let epsilon = to_dezimal(epsilon);
    let result = gamma * epsilon; 

    println!("gamma: {}, epsilon: {} => {}", gamma, epsilon, result);
    Some(())
}

fn to_dezimal(seq: Vec<u32>) -> u32 {
    let mut sum = 0;
    for (i, bin) in seq.iter().rev().enumerate() {
        sum += bin * (2 as u32).pow(i as u32);
    }
    sum
}

fn get_most_common(input: Vec<Vec<u32>>) -> Vec<u32> {
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
