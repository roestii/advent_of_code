pub fn handle_input(input: Vec<&str>) {
    let mut heights: Vec<Vec<u32>> = input.iter()
        .fold(Vec::new(), |mut heights, line| {
            heights.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
            heights
        });

    let mut lows = Vec::new();
    for i in 0..heights.len() {
        for k in 0..heights[i].len() {
            if is_low(&heights, i, k) {
                lows.push((k, i));
            }
        }
    }

    let mut sizes = lows.iter()
        .fold(Vec::new(), |mut sizes, (x, y)| {
            sizes.push(get_basin_size(&mut heights, *y, *x));
            sizes
        });
    sizes.sort();
    let result: usize = sizes.iter().rev().take(3).product();
    println!("{}", result);
}

fn get_basin_size(heights: &mut Vec<Vec<u32>>, y: usize, x: usize) -> usize {
    if heights[y][x] >= 9 {
        return 0;
    }
    heights[y][x] = 9;
    let adjacent = get_adjacent_points(x, y, heights[y].len(), heights.len());
    return 1 + adjacent.iter().fold(0, |sum, (xa, ya)| {
        return sum + get_basin_size(heights, *ya, *xa);
    });
}

fn get_adjacent_points(x: usize, y: usize, xlen: usize, ylen: usize) -> Vec<(usize, usize)> {
    let mut adjacent = Vec::new();
    if x > 0 {
        adjacent.push((x-1, y));
    }
    if x < xlen - 1 {
        adjacent.push((x+1, y));
    }
    if y > 0 {
        adjacent.push((x, y-1));
    }
    if y < ylen - 1 {
        adjacent.push((x, y+1));
    }

    adjacent
}

fn is_low(heights: &Vec<Vec<u32>>, y: usize, x: usize) -> bool {
    let adjacent: Vec<(usize, usize)> = get_adjacent_points(x, y, heights[y].len(), heights.len());
    let mut result = true;
    for (xa, ya) in &adjacent {
        if heights[y][x] >= heights[*ya][*xa] {
            result = false;
        }
    }
    result
}
