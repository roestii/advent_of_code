pub fn handle_input(input: Vec<&str>) {
    let input: Vec<u32> = input.iter().map(|x| x.parse().unwrap()).collect();
    let comp: Vec<&u32> = input.iter().skip(1).collect();
    let previous: Vec<&u32> = input.iter().take(input.len()-1).collect();

    let increases = comp.iter().zip(previous).fold(0, |acc, (element, prev)| {
        if element > &prev {
            return acc + 1;
        }
        return acc
    });

    println!("{}", increases);
}
