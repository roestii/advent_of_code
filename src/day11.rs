enum Oct {
    Flashed(u32),
    Unflashed(u32),
}

pub fn handle_input(input: Vec<&str>) {
    let mut octs: Vec<Vec<Oct>> = input.iter()
        .fold(Vec::new(), |mut octs, line| {
            octs.push(line.chars().map(|c| Oct::Unflashed(c.to_digit(10).unwrap())).collect());
            octs
        });

    for i in 1.. {
        if iteration(&mut octs) == octs.iter().flatten().count() {
            println!("{}", i);
            break;
        }
    }

}

fn iteration(octs: &mut Vec<Vec<Oct>>) -> usize {
    increase_all(octs);
    let flashes = perform_flash(octs);
    set_unflashed(octs);
    flashes
}

fn set_unflashed(octs: &mut Vec<Vec<Oct>>) {
    for row in octs {
        for oct in row {
            match oct {
                Oct::Flashed(energy) => { *oct = Oct::Unflashed(*energy) },
                _ => {},
            }
        }
    }
}

fn perform_flash(octs: &mut Vec<Vec<Oct>>) -> usize {
    let mut flashes = 0;
    for i in 0..octs.len() {
        for k in 0..octs[i].len() {
            match octs[i][k] {
                Oct::Unflashed(energy) => {
                    if energy > 9 {
                        octs[i][k] = Oct::Flashed(0);
                        flashes += 1 + increase_adjacent(octs, i, k);
                    }
                },
                _ => {},
            }
        }
    }
    flashes
}

fn increase_adjacent(octs: &mut Vec<Vec<Oct>>, i: usize, k: usize) -> usize {
    let mut flashes = 0;
    let adjacent = get_adjacent(i, k, octs.len(), octs[i].len());
    for (i, k) in adjacent.iter() {
        match octs[*i][*k] {
            Oct::Unflashed(energy) => {
                if energy + 1 > 9 {
                    octs[*i][*k] = Oct::Flashed(0);
                    flashes += 1 + increase_adjacent(octs, *i, *k);
                } else {
                    octs[*i][*k] = Oct::Unflashed(energy + 1);
                }
            },
            _ => {},
        }
    }
    flashes
}

fn get_adjacent(i: usize, k: usize, ylen: usize, xlen: usize) -> Vec<(usize, usize)> {
    let dirs: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 1),
        (1, 1),
        (1, -1),
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
    ];
    let (i, k) = (i as isize, k as isize);

    dirs.iter()
        .filter(|dir| in_bounds(i + dir.0, k + dir.1, ylen, xlen))
        .fold(Vec::new(), |mut points, dir| {
            points.push(((i + dir.0) as usize, (k + dir.1) as usize));
            points
        })

}

fn in_bounds(i: isize, k: isize, ylen: usize, xlen: usize) -> bool {
    i >= 0 && k >= 0 && i < ylen as isize && k < xlen as isize
}

fn increase_all(octs: &mut Vec<Vec<Oct>>) {
    for row in octs {
        for oct in row {
            match oct {
                Oct::Unflashed(value) => *value += 1 ,
                _ => {},
            }
        }
    }
}

fn print_octs(octs: &Vec<Vec<Oct>>) {
    octs.iter().for_each(|row| {
        row.iter().for_each(|oct| {
            match oct {
                Oct::Unflashed(energy) => print!("{}", energy),
                _ => {},
            }
        });
        println!();
    });
    println!();
}
