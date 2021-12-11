pub fn handle_input(input: Vec<&str>) {
    let horizontals: Vec<usize> = input.first()
        .unwrap()
        .split(",")
        .map(|pos| pos.parse::<usize>().unwrap())
        .collect();

    let vmin = *horizontals.iter().min().unwrap();
    let vmax = *horizontals.iter().max().unwrap();

    let min = (vmin..=vmax).fold((horizontals[0], calculate_fuel_consumption(&horizontals, vmin)), |mut min, hpos| {
            let consumption = calculate_fuel_consumption(&horizontals, hpos);
            if min.1 > consumption {
                min = (hpos, consumption); 
            }
            min
        });
    println!("{:?}", min);
}

fn calculate_fuel_consumption(horizontals: &Vec<usize>, hpos: usize) -> isize {
    let fuel_consumption = horizontals.iter().fold(0, |mut sum_diff, horizontal| {
        let diff = *horizontal as isize - hpos as isize;
        let sum: isize = diff.abs() * (diff.abs() + 1) / 2;
        sum_diff += sum;
        sum_diff
    });
    fuel_consumption
}
