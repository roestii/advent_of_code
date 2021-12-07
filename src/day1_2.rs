pub fn handle_input(input: Vec<&str>) {
    let input: Vec<u32> = input.iter().map(|x| x.parse().unwrap()).collect();
    let mut count = 0;
    
    for x in 3..input.len() { 
        let previous = calculate_chunk_sum(x-1, &input); 
        let current = calculate_chunk_sum(x, &input);

        if current > previous {
            count += 1;
        } 
    }
    println!("{}", count);
}

fn calculate_chunk_sum(index: usize, input: &Vec<u32>) -> u32 {
    let chunk = &input[index-2..=index];
    return chunk.iter().sum();
}
